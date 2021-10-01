use std::collections::HashMap;
use std::str::FromStr;

use rust_decimal::Decimal;
use tokio::sync::mpsc::Sender;
use tracing::{debug, error, trace};
use tungstenite::Message;

use crate::cache::{AnchorAction, CacheEvent};
use crate::event::{Attribute, EventDataSlim, EventTypeSlim, LogEvent};

pub async fn handle_msg(msg: Message, tx: Sender<CacheEvent>) {
    match msg {
        Message::Text(txt) => {
            trace!("Received message: {}", &txt);
            match serde_json::from_str(&txt) {
                Ok(EventTypeSlim::NewBlock { data, .. }) => handle_new_block(data, tx).await,
                Err(e) => error!(
                    "ERROR: Could not parse JSON string: {} {}",
                    e.to_string(),
                    txt
                ),
            }
        }
        Message::Binary(b) => debug!("{:?}", b),
        Message::Ping(_) => trace!("Received Ping"),
        Message::Pong(_) => trace!("Received Pong"),
        Message::Close(_) => trace!("Received close"),
    }
}

pub async fn handle_new_block(data: EventDataSlim, tx: Sender<CacheEvent>) {
    data.txs
        .into_iter()
        .flat_map(|tx| tx.logs.into_iter())
        .flat_map(|log| log.events.into_iter())
        .for_each(|event| {
            let tx = tx.clone();
            tokio::spawn(async move {
                match event {
                    LogEvent::FromContract { attributes: attrs } => {
                        let map = attribute_map(attrs);
                        handle_from_contract(map, tx).await;
                    }
                    _ => trace!("Ignoring event: {:?}", event),
                }
            });
        });
}

fn attribute_map(attrs: Vec<Attribute>) -> HashMap<String, String> {
    attrs
        .into_iter()
        .filter(|attr| attr.value.is_some())
        .fold(HashMap::new(), |mut acc, attr| {
            if let Some(value) = attr.value {
                acc.insert(attr.key.clone(), value.clone());
                acc
            } else {
                acc
            }
        })
        .clone()
}

pub async fn handle_from_contract(attrs: HashMap<String, String>, tx: Sender<CacheEvent>) {
    match attrs.get("action") {
        Some(action) => {
            trace!("Handling action: {}", action);
            match AnchorAction::from_str(action.as_ref()) {
                Ok(AnchorAction::BorrowStable) => process_borrow_stable(attrs, tx).await,
                Ok(AnchorAction::RepayStable) => process_repay_stable(attrs, tx).await,
                Ok(AnchorAction::DepositCollateral) => process_deposit_collateral(attrs, tx).await,
                Ok(AnchorAction::WithdrawCollateral) => {
                    process_withdraw_collateral(attrs, tx).await
                }
                _ => trace!("Ignoring action: {}", action),
            }
        }
        None => error!("Caught empty action. Attributes: {:?}", attrs),
    }
}

pub async fn process_borrow_stable(attrs: HashMap<String, String>, tx: Sender<CacheEvent>) {
    debug!("borrow_stable event: {:?}", attrs);
    let pair = (attrs.get("borrower"), attrs.get("borrow_amount"));
    let statement = match pair {
        (Some(borrower), Some(amount)) => {
            let parsed = amount
                .parse::<i64>()
                .map(|a| Decimal::new(a, 6))
                .expect(&format!(
                    "Error parsing borrow amount from event: {:?}",
                    attrs
                ));
            Some((borrower.clone(), parsed.clone()))
        }
        _ => None,
    };
    if let Some(statement) = statement {
        tx.send(CacheEvent::BorrowStable {
            address: statement.0,
            amount: statement.1,
        })
        .await
        .expect("Error sending BorrowStable Event")
    }
}

pub async fn process_repay_stable(attrs: HashMap<String, String>, tx: Sender<CacheEvent>) {
    debug!("repay_stable event: {:?}", attrs);
    let pair = (attrs.get("borrower"), attrs.get("repay_amount"));
    let statement = match pair {
        (Some(borrower), Some(amount)) => {
            let parsed = amount
                .parse::<i64>()
                .map(|a| Decimal::new(a, 6))
                .expect(&format!(
                    "Error parsing repay amount from event: {:?}",
                    attrs
                ));
            Some((borrower.clone(), parsed.clone()))
        }
        _ => None,
    };
    if let Some(statement) = statement {
        tx.send(CacheEvent::RepayStable {
            address: statement.0,
            amount: statement.1,
        })
        .await
        .expect("Error sending RepayStable Event");
    }
}

pub async fn process_deposit_collateral(attrs: HashMap<String, String>, tx: Sender<CacheEvent>) {
    debug!("deposit_collateral event: {:?}", attrs);
    let pair = (attrs.get("borrower"), attrs.get("amount"));
    let statement = match pair {
        (Some(borrower), Some(amount)) => {
            let parsed = amount
                .parse::<i64>()
                .map(|a| Decimal::new(a, 6))
                .expect(&format!(
                    "Error parsing deposit amount from event: {:?}",
                    attrs
                ));
            Some((borrower.clone(), parsed.clone()))
        }
        _ => None,
    };
    if let (Some(statement), Some(contract_address)) = (statement, attrs.get("contract_address")) {
        tx.send(CacheEvent::DepositCollateral {
            address: statement.0,
            amount: statement.1,
            contract_address: contract_address.clone(),
        })
        .await
        .expect("Error sending DepositCollateral Event");
    }
}

pub async fn process_withdraw_collateral(attrs: HashMap<String, String>, tx: Sender<CacheEvent>) {
    debug!("withdraw_collateral event: {:?}", attrs);
    let pair = (attrs.get("borrower"), attrs.get("amount"));
    let statement = match pair {
        (Some(borrower), Some(amount)) => {
            let parsed = amount
                .parse::<i64>()
                .map(|a| Decimal::new(a, 6))
                .expect(&format!(
                    "Error parsing withdraw amount from event: {:?}",
                    attrs
                ));
            Some((borrower.clone(), parsed.clone()))
        }
        _ => None,
    };
    if let (Some(statement), Some(contract_address)) = (statement, attrs.get("contract_address")) {
        tx.send(CacheEvent::WithdrawCollateral {
            address: statement.0,
            amount: statement.1,
            contract_address: contract_address.clone(),
        })
        .await
        .expect("Error sending WithdrawCollateral Event");
    }
}
