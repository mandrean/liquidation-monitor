pub mod client;

#[non_exhaustive]
pub struct Contracts;
impl<'a> Contracts {
    pub const MARKET: &'a str = "terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s";
    pub const OVERSEER: &'a str = "terra1tmnqgvg567ypvsvk6rwsga3srp7e3lg6u0elp8";
    pub const BETH: &'a str = "terra1dzhzukyezv0etz22ud940z7adyv7xgcjkahuun";
    pub const BLUNA: &'a str = "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp";
    pub const ORACLE: &'a str = "terra1cgg6yef7qcdm070qftghfulaxmllgmvk77nc7t";
}

mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}

#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;
    use crate::mantle::Contracts;
    use anyhow::{anyhow, Result};
    use cynic::{Operation, QueryBuilder};
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;

    pub trait ToJson {
        fn to_json(&self) -> String
        where
            Self: Serialize,
        {
            serde_json::to_string(self).expect("Error serializing object")
        }
    }
    pub trait FromJson<R> {
        fn from_json<'a, T>(json: &'a T) -> Result<R>
        where
            T: AsRef<str> + ToString + Display,
            R: Deserialize<'a>,
        {
            serde_json::from_str(json.as_ref())
                .map_err(|_| anyhow!("Error deserializing {}", std::any::type_name::<T>()))
        }
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct BorrowLiquidationPriceQueryArguments {
        pub market_contract: String,
        pub market_borrower_info_query: String,
        pub overseer_contract: String,
        pub overseer_borrowlimit_query: String,
        pub overseer_collaterals_query: String,
        pub overseer_whitelist_query: String,
        pub oracle_contract: String,
        pub oracle_price_query: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct BorrowerInfo {
        pub borrower: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Whitelist {
        pub collateral_token: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Price {
        pub base: String,
        pub quote: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct MarketBorrowerInfoQuery {
        pub borrower_info: BorrowerInfo,
    }

    impl ToJson for MarketBorrowerInfoQuery {}
    impl MarketBorrowerInfoQuery {
        pub fn new<T: AsRef<str> + ToString>(borrower: T) -> MarketBorrowerInfoQuery {
            MarketBorrowerInfoQuery {
                borrower_info: BorrowerInfo {
                    borrower: borrower.to_string(),
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct OverseerBorrowLimitQuery {
        pub borrow_limit: BorrowerInfo,
    }

    impl ToJson for OverseerBorrowLimitQuery {}
    impl OverseerBorrowLimitQuery {
        pub fn new<T: AsRef<str> + ToString>(borrower: T) -> OverseerBorrowLimitQuery {
            OverseerBorrowLimitQuery {
                borrow_limit: BorrowerInfo {
                    borrower: borrower.to_string(),
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct OverseerCollateralsQuery {
        pub collaterals: BorrowerInfo,
    }

    impl ToJson for OverseerCollateralsQuery {}
    impl OverseerCollateralsQuery {
        pub fn new<T: AsRef<str> + ToString>(borrower: T) -> OverseerCollateralsQuery {
            OverseerCollateralsQuery {
                collaterals: BorrowerInfo {
                    borrower: borrower.to_string(),
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct OverseerWhitelistQuery {
        pub whitelist: Whitelist,
    }

    impl ToJson for OverseerWhitelistQuery {}
    impl OverseerWhitelistQuery {
        pub fn new<T: AsRef<str> + ToString>(collateral_token: T) -> OverseerWhitelistQuery {
            OverseerWhitelistQuery {
                whitelist: Whitelist {
                    collateral_token: collateral_token.to_string(),
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct OraclePriceQuery {
        pub price: Price,
    }

    impl ToJson for OraclePriceQuery {}
    impl OraclePriceQuery {
        pub fn new<T: AsRef<str> + ToString>(base: T, quote: T) -> OraclePriceQuery {
            OraclePriceQuery {
                price: {
                    Price {
                        base: base.to_string(),
                        quote: quote.to_string(),
                    }
                },
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Elem {
        pub collateral_token: String,
        pub custody_contract: String,
        pub max_ltv: String,
        pub name: String,
        pub symbol: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MarketBorrowerInfo {
        pub borrower: String,
        pub interest_index: String,
        pub loan_amount: String,
        pub pending_rewards: String,
        pub reward_index: String,
    }
    impl FromJson<MarketBorrowerInfo> for MarketBorrowerInfo {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct OverseerBorrowLimit {
        pub borrow_limit: String,
        pub borrower: String,
    }
    impl FromJson<OverseerBorrowLimit> for OverseerBorrowLimit {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct OverseerCollaterals {
        pub borrower: String,
        pub collaterals: Vec<Vec<String>>,
    }
    impl FromJson<OverseerCollaterals> for OverseerCollaterals {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct OverseerWhitelist {
        pub elems: Vec<Elem>,
    }
    impl FromJson<OverseerWhitelist> for OverseerWhitelist {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct OraclePriceInfo {
        pub last_updated_base: i64,
        pub last_updated_quote: i64,
        pub rate: String,
    }
    impl FromJson<OraclePriceInfo> for OraclePriceInfo {}

    #[derive(cynic::FragmentArguments, Debug, Clone)]
    pub struct BorrowStableTxHistoryQueryArguments {
        pub height_range: Option<Vec<Option<i32>>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "RootQuery",
        argument_struct = "BorrowStableTxHistoryQueryArguments"
    )]
    pub struct BorrowStableTxHistoryQuery {
        #[arguments(transaction_type = "borrow_stable", height_range = args.height_range.clone(), limit = 500)]
        pub transaction_history: Option<Vec<Option<TransactionHistory>>>,
        pub last_synced_height: Option<i32>,
    }
    impl BorrowStableTxHistoryQuery {
        pub fn build_query(from: i32, to: i32) -> Operation<'static, BorrowStableTxHistoryQuery> {
            BorrowStableTxHistoryQuery::build(&BorrowStableTxHistoryQueryArguments {
                height_range: Some(vec![Some(from), Some(to)]),
            })
        }
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct TransactionHistory {
        pub height: Option<i32>,
        pub out_denom: Option<String>,
        pub out_amount: Option<String>,
        pub timestamp: Option<i32>,
        pub transaction_type: Option<String>,
        pub address: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[cynic(
        graphql_type = "RootQuery",
        argument_struct = "BorrowLiquidationPriceQueryArguments"
    )]
    pub struct BorrowLiquidationPriceQuery {
        #[arguments(contract_address = &args.market_contract, query_msg = &args.market_borrower_info_query)]
        #[cynic(rename = "WasmContractsContractAddressStore", alias)]
        pub market_borrower_info: Option<GetWasmContractsContractAddressStorePayload>,

        #[arguments(contract_address = &args.overseer_contract, query_msg = &args.overseer_borrowlimit_query)]
        #[cynic(rename = "WasmContractsContractAddressStore", alias)]
        pub overseer_borrow_limit: Option<GetWasmContractsContractAddressStorePayload>,

        #[arguments(contract_address = &args.overseer_contract, query_msg = &args.overseer_collaterals_query)]
        #[cynic(rename = "WasmContractsContractAddressStore", alias)]
        pub overseer_collaterals: Option<GetWasmContractsContractAddressStorePayload>,

        #[arguments(contract_address = &args.overseer_contract, query_msg = &args.overseer_whitelist_query)]
        #[cynic(rename = "WasmContractsContractAddressStore", alias)]
        pub overseer_whitelist: Option<GetWasmContractsContractAddressStorePayload>,

        #[arguments(contract_address = &args.oracle_contract, query_msg = &args.oracle_price_query)]
        #[cynic(rename = "WasmContractsContractAddressStore", alias)]
        pub oracle_price_info: Option<GetWasmContractsContractAddressStorePayload>,
    }
    impl BorrowLiquidationPriceQuery {
        pub fn build_query<T>(borrower: T) -> Operation<'static, BorrowLiquidationPriceQuery>
        where
            T: AsRef<str> + ToString + Display,
        {
            BorrowLiquidationPriceQuery::build(&BorrowLiquidationPriceQueryArguments {
                market_contract: Contracts::MARKET.to_string(),
                market_borrower_info_query: MarketBorrowerInfoQuery::new(&borrower).to_json(),
                overseer_contract: Contracts::OVERSEER.to_string(),
                overseer_borrowlimit_query: OverseerBorrowLimitQuery::new(&borrower).to_json(),
                overseer_collaterals_query: OverseerCollateralsQuery::new(&borrower).to_json(),
                overseer_whitelist_query: OverseerWhitelistQuery::new(Contracts::BLUNA).to_json(),
                oracle_contract: Contracts::ORACLE.to_string(),
                oracle_price_query: OraclePriceQuery::new(Contracts::BLUNA, "uusd").to_json(),
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case", untagged)]
    pub enum WasmContractResult {
        MarketBorrowerInfo {
            borrower: String,
            interest_index: String,
            loan_amount: String,
            pending_rewards: String,
            reward_index: String,
        },
        OverseerBorrowLimit {
            borrow_limit: String,
            borrower: String,
        },
        OverseerCollaterals {
            borrower: String,
            collaterals: Vec<Vec<String>>,
        },
        OverseerWhitelist {
            elems: Vec<Elem>,
        },
        OraclePricenfo {
            last_updated_base: i64,
            last_updated_quote: i64,
            rate: String,
        },
    }

    #[derive(cynic::QueryFragment, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GetWasmContractsContractAddressStorePayload {
        pub result: Option<String>,
    }

    /*
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GetWasmContractsContractAddressStorePayload {
        pub result: Option<WasmContractResult>,
    }

    impl cynic::Scalar<WasmContractResult> for GetWasmContractsContractAddressStorePayload {
        type Deserialize = GetWasmContractsContractAddressStorePayload;

        fn from_deserialize(x: Self::Deserialize) -> Result<Self, cynic::DecodeError> {
            Ok(x)
        }
    }*/

    // error: Cynic does not understand this type. Only un-parameterised types, Vecs, Options & Box are accepted currently.
    /*
            #[arguments(contract_address = &args.market_contract, query_msg = &args.market_borrower_info_query)]
            #[cynic(rename = "WasmContractsContractAddressStore", alias)]
            pub market_borrower_info: Option<GetWasmContractsContractAddressStorePayload<MarketBorrowerInfo>>,
    */
    /*
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GetWasmContractsContractAddressStorePayload<T: Serialize> {
        pub result: Option<T>,
    }

    impl<T: Serialize> cynic::Scalar<T> for GetWasmContractsContractAddressStorePayload<T> {
        type Deserialize = GetWasmContractsContractAddressStorePayload<T>;

        fn from_deserialize(x: Self::Deserialize) -> Result<Self, cynic::DecodeError> {
            Ok(x)
        }
    }
    */
}
