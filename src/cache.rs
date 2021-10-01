use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;
use std::string::ToString;
use std::sync::Arc;

use crate::anchor;
use crate::mantle::client::{MantleClient, MantleExt};
use crate::mantle::Contracts;
use anyhow::{Error, Result};
use cached::proc_macro::cached;
use cached::TimedCache;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumString, ToString};
use tokio::sync::{mpsc::Receiver, RwLock};
use tracing::{debug, warn};

pub type Borrowers = Arc<RwLock<BTreeMap<String, Loan>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loan {
    pub amount: Decimal,
    pub collaterals: HashMap<String, Decimal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidationLevel {
    pub bluna_vol: Decimal,
    pub beth_vol: Decimal,
    pub borrowers: HashSet<String>,
}

#[derive(ToString, EnumDiscriminants, Debug, Clone, PartialEq)]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(name(AnchorAction))]
pub enum CacheEvent {
    #[strum_discriminants(strum(serialize = "borrow_stable"))]
    BorrowStable { address: String, amount: Decimal },
    #[strum_discriminants(strum(serialize = "repay_stable"))]
    RepayStable { address: String, amount: Decimal },
    #[strum_discriminants(strum(serialize = "deposit_collateral"))]
    DepositCollateral {
        address: String,
        amount: Decimal,
        contract_address: String,
    },
    #[strum_discriminants(strum(serialize = "withdraw_collateral"))]
    WithdrawCollateral {
        address: String,
        amount: Decimal,
        contract_address: String,
    },
}

/// A cache containing data about borrowers on Anchor and their loans
pub struct AnchorCache {
    pub borrowers: Borrowers,
}

impl AnchorCache {
    pub fn new() -> Self {
        AnchorCache {
            borrowers: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub async fn seed_borrowers<P: AsRef<str>>(&self, path: P) -> Result<()> {
        let seed: BTreeMap<String, Loan> =
            serde_json::from_str(read_to_string(path.as_ref())?.as_ref())
                .map_err(|e| Error::from(e))?;
        self.borrowers.clone().write().await.extend(seed);
        Ok(())
    }

    /// Initiate the listener. Populates the cache based on the incoming events.
    pub fn init_listener(&self, mut rx: Receiver<CacheEvent>) {
        let borrowers = self.borrowers.clone();

        tokio::spawn(async move {
            while let Some(event) = &rx.recv().await {
                debug!("Received Cache Event: {}", event.to_string());
                match event {
                    CacheEvent::BorrowStable { address, amount }
                    | CacheEvent::RepayStable { address, amount } => {
                        match borrowers.write().await.get_mut(address) {
                            Some(loan) => {
                                match event {
                                    CacheEvent::BorrowStable { .. } => {
                                        // increase loan amount
                                        loan.amount += amount;
                                    }
                                    CacheEvent::RepayStable { .. } => {
                                        // decrease loan amount
                                        loan.amount -= amount;
                                    }
                                    _ => {}
                                }
                            }
                            None => {},
                            // FIXME: Columbus-5 broke this
                            // None => match MantleClient::query_loan(address).await {
                            //     Ok(loan) => {
                            //         borrowers.write().await.insert(
                            //             address.clone(),
                            //             Loan {
                            //                 amount: loan.amount,
                            //                 collaterals: loan.collaterals,
                            //             },
                            //         );
                            //     }
                            //     Err(e) => warn!("{}", e),
                            // },
                        }
                    }
                    CacheEvent::DepositCollateral {
                        address,
                        amount,
                        contract_address,
                    }
                    | CacheEvent::WithdrawCollateral {
                        address,
                        amount,
                        contract_address,
                    } => match borrowers.write().await.get_mut(address) {
                        Some(loan) => {
                            let beth = loan.collaterals.get(Contracts::BETH);
                            let bluna = loan.collaterals.get(Contracts::BLUNA);

                            match event {
                                CacheEvent::DepositCollateral { .. } => {
                                    // increase collaterals
                                    match (beth, bluna, contract_address.as_ref()) {
                                        (Some(beth), _, Contracts::BETH) => {
                                            let new_amount = *beth + *amount;
                                            loan.collaterals
                                                .insert(Contracts::BETH.to_string(), new_amount);
                                        }
                                        (_, Some(bluna), Contracts::BLUNA) => {
                                            let new_amount = *bluna + *amount;
                                            loan.collaterals
                                                .insert(Contracts::BLUNA.to_string(), new_amount);
                                        }
                                        (_, _, _) => {}
                                    }
                                }
                                CacheEvent::WithdrawCollateral { .. } => {
                                    // decrease collaterals
                                    match (beth, bluna, contract_address.as_ref()) {
                                        (Some(beth), _, Contracts::BETH) => {
                                            let new_amount = *beth - *amount;
                                            loan.collaterals
                                                .insert(Contracts::BETH.to_string(), new_amount);
                                        }
                                        (_, Some(bluna), Contracts::BLUNA) => {
                                            let new_amount = *bluna - *amount;
                                            loan.collaterals
                                                .insert(Contracts::BLUNA.to_string(), new_amount);
                                        }
                                        (_, _, _) => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => {},
                        // FIXME: Columbus-5 broke this
                        // None => match MantleClient::query_loan(address).await {
                        //     Ok(loan) => {
                        //         borrowers.write().await.insert(
                        //             address.clone(),
                        //             Loan {
                        //                 amount: loan.amount,
                        //                 collaterals: loan.collaterals,
                        //             },
                        //         );
                        //     }
                        //     Err(e) => warn!("{}", e),
                        // },
                    },
                }
            }
        });
    }
}

#[cached(
    result = true,
    type = "TimedCache<String, String>",
    create = "{ TimedCache::with_lifespan_and_capacity(10, 100000) }",
    convert = r#"{ "borrowers".to_string() }"#
)]
/// TLRU cache of the serialized borrower data
pub fn cached_borrowers(borrowers: BTreeMap<String, Loan>) -> Result<String> {
    serde_json::to_string(&borrowers).map_err(|e| Error::from(e))
}

#[cached(
    result = true,
    type = "TimedCache<String, String>",
    create = "{ TimedCache::with_lifespan_and_capacity(10, 100000) }",
    convert = r#"{ beth_price.clone().to_string() }"#
)]
/// TLRU cache of the calculated serialized liquidation levels
pub fn cached_liquidations(
    borrowers: BTreeMap<String, Loan>,
    beth_price: &Decimal,
) -> Result<String> {
    let data = borrowers.iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<Decimal, LiquidationLevel>, (address, loan)| {
            let beth = loan
                .collaterals
                .get(Contracts::BETH)
                .map(|c| c.clone())
                .unwrap_or(Decimal::new(0, 0));
            let max_ltv = Decimal::new(6, 1);

            if let Some(bluna) = loan.collaterals.get(Contracts::BLUNA) {
                let liq_price = anchor::liquidation_price_multi(
                    &loan.amount,
                    &beth,
                    beth_price,
                    bluna,
                    &max_ltv,
                );
                let liq_key = liq_price.round_dp(1);

                match acc.get_mut(&liq_key) {
                    Some(liq_level) if liq_price.is_sign_positive() => {
                        liq_level.bluna_vol += bluna.clone();
                        liq_level.beth_vol += beth.clone();
                        liq_level.borrowers.insert(address.clone());
                    }
                    None if liq_price.is_sign_positive() => {
                        let mut borrowers = HashSet::new();
                        borrowers.insert(address.clone());
                        acc.insert(
                            liq_key,
                            LiquidationLevel {
                                bluna_vol: bluna.clone(),
                                beth_vol: beth.clone(),
                                borrowers,
                            },
                        );
                    }
                    _ => {}
                }
            }
            acc
        },
    );
    serde_json::to_string(&data).map_err(|e| Error::from(e))
}
