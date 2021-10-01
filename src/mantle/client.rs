use std::collections::HashMap;
use std::fmt::Display;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use cynic::http::SurfExt;
use rust_decimal::prelude::*;
use surf::RequestBuilder;
use tracing::{debug, error};

use crate::anchor;
use crate::cache::Loan;
use crate::mantle::{
    queries::{BorrowLiquidationPriceQuery, FromJson, MarketBorrowerInfo, OverseerCollaterals},
    Contracts,
};

pub type MantleClient = RequestBuilder;

const MANTLE_HOST: &str = "https://mantle.anchorprotocol.com";

#[async_trait]
pub trait MantleExt {
    fn default() -> MantleClient;
    async fn query_loan<T: AsRef<str> + ToString + Display + Send + Sync>(
        borrower: T,
    ) -> Result<Loan>;
    async fn query_liquidation_price<T: AsRef<str> + ToString + Display + Send + Sync>(
        borrower: T,
    ) -> Result<Option<Decimal>>;
}

#[async_trait]
impl MantleExt for MantleClient {
    fn default() -> MantleClient {
        surf::post(MANTLE_HOST)
    }

    async fn query_loan<T: AsRef<str> + ToString + Display + Send + Sync>(
        borrower: T,
    ) -> Result<Loan> {
        if !borrower.as_ref().starts_with("terra") {
            return Err(anyhow!("'borrower' must be a valid Terra address"));
        }

        let data = surf::post(MANTLE_HOST)
            .run_graphql(BorrowLiquidationPriceQuery::build_query(&borrower))
            .await
            .map_err(|e| e.into_inner())
            .map(|res| res.data);

        match data {
            Ok(Some(q)) => parse_loan(q),
            Ok(None) => Err(anyhow!(
                "Couldn't fetch loan info of borrower: {}",
                &borrower
            )),
            Err(e) => Err(e),
        }
    }

    async fn query_liquidation_price<T: AsRef<str> + ToString + Display + Send + Sync>(
        borrower: T,
    ) -> Result<Option<Decimal>> {
        if !borrower.as_ref().starts_with("terra") {
            return Err(anyhow!("'borrower' must be a valid Terra address"));
        }

        // let data = self.run_graphql
        let data = surf::post(MANTLE_HOST)
            .run_graphql(BorrowLiquidationPriceQuery::build_query(borrower))
            .await
            .map_err(|e| e.into_inner())
            .map(|res| res.data);

        match data {
            Ok(Some(q)) => liquidation_price(q),
            //Ok(Some(q)) => liquidation_price_multi(q),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn parse_loan_amount(q: &BorrowLiquidationPriceQuery) -> Result<Option<Decimal>, Error> {
    q.market_borrower_info
        .as_ref()
        .and_then(|p| p.result.as_ref())
        .map(|res| MarketBorrowerInfo::from_json(&res))
        .transpose()
        .expect("JSON parsing error")
        .map(|info| {
            i64::from_str(info.loan_amount.as_ref())
                .map_err(|e| anyhow::Error::from(e))
                .map(|i| Decimal::new(i, 6))
        })
        .transpose()
}

fn parse_collateral(
    q: &BorrowLiquidationPriceQuery,
    collateral_addr: &str,
) -> Result<Option<Decimal>, Error> {
    q.overseer_collaterals
        .as_ref()
        .and_then(|p| p.result.as_ref())
        .map(|result| OverseerCollaterals::from_json(&result))
        .transpose()
        .and_then(|c| {
            if c.is_none() {
                return Ok(None);
            }

            let collaterals =
                c.unwrap()
                    .collaterals
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, vec| {
                        let mut iter = vec.into_iter();
                        match (iter.next().clone(), iter.next()) {
                            (Some(col_addr), Some(amount)) if col_addr.starts_with("terra") => {
                                acc.insert(col_addr.clone().to_owned(), amount.clone().to_owned());
                            }
                            (Some(amount), Some(col_addr)) if col_addr.starts_with("terra") => {
                                acc.insert(col_addr.clone().to_owned(), amount.clone().to_owned());
                            }
                            (a, b) => error!("Missing something: {:?} {:?}", a, b),
                        }
                        acc
                    });

            collaterals
                .get(collateral_addr)
                .map(|s| {
                    Decimal::from_str(&s)
                        .map_err(|e| anyhow::Error::from(e))
                        .map(|mut d| {
                            d.set_scale(6).unwrap();
                            d
                        })
                })
                .transpose()
        })
}

pub fn parse_loan(q: BorrowLiquidationPriceQuery) -> Result<Loan> {
    let loan_amount = parse_loan_amount(&q);
    let beth_collateral = parse_collateral(&q, Contracts::BETH);
    let bluna_collateral = parse_collateral(&q, Contracts::BLUNA);

    match (&loan_amount, &beth_collateral, &bluna_collateral) {
        (Ok(Some(amount)), Ok(beth), Ok(Some(bluna))) => {
            let mut collaterals = HashMap::new();
            collaterals.insert(Contracts::BLUNA.to_string(), bluna.clone());
            if let Some(beth) = beth {
                collaterals.insert(Contracts::BETH.to_string(), beth.clone());
            }
            Ok(Loan {
                amount: *amount,
                collaterals,
            })
        }
        (_, _, _) => {
            debug!(
                "Loan amount result: {:?}\nCollateral amount result: {:?}\n",
                loan_amount, bluna_collateral
            );
            Err(anyhow!("Liquidation error"))
        }
    }
}

pub fn liquidation_price(q: BorrowLiquidationPriceQuery) -> Result<Option<Decimal>> {
    let loan_amount = parse_loan_amount(&q);
    let bluna_collateral = parse_collateral(&q, Contracts::BLUNA);
    let max_ltv = Decimal::new(6, 1);

    match (&loan_amount, &bluna_collateral) {
        (Ok(Some(a)), Ok(Some(c))) => Ok(Some(anchor::liquidation_price(a, c, &max_ltv))),
        (_, _) => {
            debug!(
                "Loan amount result: {:?}\nCollateral amount result: {:?}\n",
                loan_amount, bluna_collateral
            );
            Err(anyhow!("Liquidation price error"))
        }
    }
}

pub fn liquidation_price_multi(
    q: BorrowLiquidationPriceQuery,
    beth_price: Decimal,
) -> Result<Option<Decimal>> {
    let loan_amount = parse_loan_amount(&q);
    let beth_collateral = parse_collateral(&q, Contracts::BETH);
    let bluna_collateral = parse_collateral(&q, Contracts::BLUNA);
    let max_ltv = Decimal::new(6, 1);

    match (&loan_amount, &beth_collateral, &bluna_collateral) {
        (Ok(Some(amount)), Ok(Some(beth)), Ok(Some(bluna))) => Ok(Some(
            anchor::liquidation_price_multi(amount, beth, &beth_price, bluna, &max_ltv),
        )),
        // fallback to default
        (Ok(Some(amount)), Ok(None), Ok(Some(bluna))) => {
            Ok(Some(anchor::liquidation_price(amount, bluna, &max_ltv)))
        }
        (_, _, _) => {
            debug!(
                "Loan amount result: {:?}\nCollateral amount result: {:?}\n",
                loan_amount, bluna_collateral
            );
            Err(anyhow!("Liquidation error"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn throws_error_on_bad_address() {
        let price = MantleClient::query_liquidation_price("abcd").await;
        assert!(!price.is_err());
    }
}
