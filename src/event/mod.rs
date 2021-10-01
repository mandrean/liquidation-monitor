pub mod handler;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hash(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address(pub String);

#[derive(Display, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventType {
    #[serde(rename = "new_block")]
    NewBlock { chain_id: Id, data: EventData },
}

#[derive(Display, Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum EventTypeSlim {
    #[serde(rename = "new_block")]
    NewBlock { chain_id: Id, data: EventDataSlim },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    pub block: Block,
    pub result_begin_block: Option<BeginBlock>,
    pub result_end_block: Option<EndBlock>,
    #[serde(default)]
    pub txs: Vec<Tx>,
    pub supply: Amounts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventDataSlim {
    #[serde(default)]
    pub txs: Vec<TxSlim>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub data: Data,
    pub evidence: EvidenceUpdates,
    pub last_commit: LastCommit,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub version: Version,
    pub chain_id: Id,
    pub height: Option<String>,
    pub time: Option<String>,
    pub last_block_id: BlockId,
    pub last_commit_hash: Hash,
    pub data_hash: Hash,
    pub validators_hash: Hash,
    pub next_validators_hash: Hash,
    pub consensus_hash: Hash,
    pub app_hash: Hash,
    pub last_results_hash: Hash,
    pub evidence_hash: Hash,
    pub proposer_address: Option<Address>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub block: Option<String>,
    pub app: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parts {
    pub total: Option<u64>,
    pub hash: Hash,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub txs: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceUpdates {
    pub evidence: Option<Vec<::serde_json::Value>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastCommit {
    pub height: Option<String>,
    pub round: Option<u64>,
    pub block_id: BlockId,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockId {
    pub hash: Hash,
    pub parts: Parts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub block_id_flag: u64,
    pub validator_address: Option<Address>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeginBlock {
    #[serde(default)]
    pub events: Vec<LogEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    BorrowStable,
    ExecuteContract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndBlock {
    pub consensus_param_updates: ConsensusParamUpdates,
    #[serde(default)]
    pub events: Vec<LogEvent>,
    #[serde(default)]
    pub validator_updates: Vec<ValidatorUpdate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsensusParamUpdates {
    pub block: BlockParamUpdates,
    pub evidence: EvidenceParamUpdates,
    pub validator: Validator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockParamUpdates {
    pub max_bytes: String,
    pub max_gas: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceParamUpdates {
    pub max_age_duration: String,
    pub max_age_num_blocks: String,
    pub max_bytes: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Validator {
    pub pub_key_types: Vec<PubKeyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorUpdate {
    pub power: Option<String>,
    pub pub_key: PubKeyWrapper,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx {
    pub auth_info: AuthInfo,
    pub body: Body,
    pub code: u64,
    pub codespace: Option<String>,
    pub data: Option<String>,
    pub height: Option<String>,
    pub info: String,
    #[serde(default)]
    pub logs: Vec<Log>,
    pub raw_log: Option<String>,
    pub signatures: Vec<String>,
    pub gas_wanted: Option<String>,
    pub gas_used: Option<String>,
    pub tx: TxData,
    pub txhash: Hash,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TxSlim {
    pub logs: Vec<LogSlim>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthInfo {
    pub fee: Fee,
    pub signer_infos: Vec<SignerInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fee {
    pub amount: Vec<Amount>,
    pub gas_limit: String,
    pub granter: String,
    pub payer: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignerInfo {
    pub mode_info: ModeInfo,
    pub public_key: PublicKey,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeInfo {
    pub single: Single,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Single {
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum PublicKey {
    #[serde(rename = "/cosmos.crypto.secp256k1.PubKey")]
    CosmosCryptoSecp256K1PubKey { key: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub extension_options: Vec<::serde_json::Value>,
    pub memo: Option<String>,
    pub messages: Vec<Message>,
    pub non_critical_extension_options: Vec<::serde_json::Value>,
    pub timeout_height: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Message {
    #[serde(rename = "/terra.oracle.v1beta1.MsgAggregateExchangeRatePrevote")]
    MsgAggregateExchangeRatePrevote {
        feeder: String,
        validator: String,
        hash: Option<String>,
    },

    #[serde(rename = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward")]
    MsgWithdrawDelegatorReward {
        delegator_address: Option<String>,
        validator_address: Option<String>,
    },

    #[serde(rename = "/terra.oracle.v1beta1.MsgAggregateExchangeRateVote")]
    MsgAggregateExchangeRateVote {
        salt: Option<String>,
        exchange_rates: Option<String>,
        feeder: String,
        validator: String,
        hash: Option<String>,
    },

    #[serde(rename = "/terra.wasm.v1beta1.MsgExecuteContract")]
    MsgExecuteContract {
        coins: Vec<Coin>,
        contract: Option<String>,
        //execute_msg: ExecuteMsg, // TODO: Figure this out
        execute_msg: ::serde_json::Value,
        sender: Option<String>,
    },

    #[serde(rename = "/terra.market.v1beta1.MsgSwap")]
    MsgSwap {
        ask_denom: String,
        offer_coin: Coin,
        trader: String,
    },

    #[serde(rename = "/cosmos.bank.v1beta1.MsgSend")]
    MsgSend {
        amount: Option<Amounts>,
        from_address: Option<String>,
        to_address: Option<String>,
    },

    #[serde(rename = "/cosmos.staking.v1beta1.MsgBeginRedelegate")]
    MsgBeginRedelegate {
        amount: Option<Amounts>,
        delegator_address: Option<String>,
        validator_dst_address: Option<String>,
        validator_src_address: Option<String>,
    },

    #[serde(rename = "/cosmos.staking.v1beta1.MsgDelegate")]
    MsgDelegate {
        amount: Option<Amounts>,
        delegator_address: Option<String>,
        validator_address: Option<String>,
    },
    #[serde(rename = "/cosmos.staking.v1beta1.MsgUndelegate")]
    MsgUndelegate {
        amount: Option<Amounts>,
        delegator_address: Option<String>,
        validator_address: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        amount: String,
        recipient: String,
    },
    Relay {
        rates: Vec<String>,
        request_ids: Vec<u64>,
        resolve_times: Vec<u64>,
        symbols: Vec<String>,
    },
    Swap {
        belief_price: String,
        max_spread: String,
        offer_asset: Asset,
    },
    Withdraw(String),
    Send {
        amount: String,
        contract: String,
        msg: String,
    },
    FeedPrice {
        prices: Vec<Vec<String>>,
    },
    IncreaseAllowance {
        amount: String,
        expires: Expires,
        spender: String,
    },
    ProvideLiquidity {
        assets: Vec<Asset>,
    },
    WithdrawVotingTokens {
        amount: String,
    },
    DepositStable(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expires {
    Never(()),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub amount: String,
    pub info: AssetInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeToken {
    pub denom: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub msg_index: u64,
    pub log: Option<String>,
    #[serde(default)]
    pub events: Vec<LogEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogSlim {
    #[serde(default)]
    pub events: Vec<LogEvent>,
}

/*
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEvent {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LogEvent {
    AggregatePrevote { attributes: Vec<Attribute> },
    AggregateVote { attributes: Vec<Attribute> },
    Burn { attributes: Vec<Attribute> },
    Coinbase { attributes: Vec<Attribute> },
    CoinSpent { attributes: Vec<Attribute> },
    CoinReceived { attributes: Vec<Attribute> },
    Commission { attributes: Vec<Attribute> },
    CompleteUnbonding { attributes: Vec<Attribute> },
    Delegate { attributes: Vec<Attribute> },
    ExecuteAuthorization { attributes: Vec<Attribute> },
    ExecuteContract { attributes: Vec<Attribute> },
    ExchangeRateUpdate { attributes: Vec<Attribute> },
    FromContract { attributes: Vec<Attribute> },
    InstantiateContract { attributes: Vec<Attribute> },
    Liveness { attributes: Vec<Attribute> },
    Message { attributes: Vec<Attribute> },
    Mint { attributes: Vec<Attribute> },
    ProposerReward { attributes: Vec<Attribute> },
    Redelegate { attributes: Vec<Attribute> },
    Rewards { attributes: Vec<Attribute> },
    StoreCode { attributes: Vec<Attribute> },
    Swap { attributes: Vec<Attribute> },
    Transfer { attributes: Vec<Attribute> },
    Unbond { attributes: Vec<Attribute> },
    Wasm { attributes: Vec<Attribute> },
    WithdrawCommission { attributes: Vec<Attribute> },
    WithdrawRewards { attributes: Vec<Attribute> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    pub index: Option<bool>,
    pub key: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new<T: AsRef<str> + ToString>(index: bool, key: T, value: T) -> Attribute {
        Attribute {
            index: Some(index),
            key: key.to_string(),
            value: Some(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum TxData {
    #[serde(rename = "/cosmos.tx.v1beta1.Tx")]
    Tx {
        auth_info: AuthInfo,
        body: Body,
        signatures: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TxDataOld {
    #[serde(rename = "core/StdFee")]
    StdFee(StdFee),

    #[serde(rename = "core/StdTx")]
    StdTx {
        #[serde(default)]
        msg: Vec<Msg>,
        fee: StdFee,
        #[serde(default)]
        signatures: Vec<StdSignature>,
        memo: Option<String>,
    },

    #[serde(rename = "core/StdSignature")]
    StdSignature(StdSignature),

    #[serde(rename = "core/StdSignDoc")]
    StdSignDoc {
        account_number: Option<String>,
        chain_id: Option<Id>,
        fee: Option<String>,
        memo: Option<String>,
        msgs: Vec<String>,
        sequence: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    #[serde(rename = "bank/MsgSend")]
    MsgSend {
        from_address: Option<String>,
        to_address: Option<String>,
        #[serde(default)]
        amount: Vec<Amount>,
    },

    #[serde(rename = "bank/MsgMultiSend")]
    MsgMultiSend {
        #[serde(default)]
        inputs: Vec<IO>,
        #[serde(default)]
        outputs: Vec<IO>,
    },

    #[serde(rename = "distribution/MsgWithdrawDelegationReward")]
    MsgWithdrawDelegationReward {
        delegator_address: Option<Address>,
        validator_address: Option<Address>,
    },

    #[serde(rename = "distribution/MsgWithdrawValidatorCommission")]
    MsgWithdrawValidatorCommission { validator_address: Option<Address> },

    #[serde(rename = "market/MsgSwap")]
    MsgSwap {
        sender: Option<Address>,
        contract: Option<Address>,
        execute_msg: Option<String>,
        #[serde(default)]
        coins: Vec<Coin>,
        trader: Option<Address>,
        offer_coin: Option<Coin>,
        ask_denom: Option<String>,
        #[serde(default)]
        inputs: Vec<IO>,
        #[serde(default)]
        outputs: Vec<IO>,
        delegator_address: Option<Address>,
        validator_address: Option<Address>,
    },

    #[serde(rename = "msgauth/MsgExecAuthorized")]
    MsgExecAuthorized {
        grantee: Option<Address>,
        #[serde(default)]
        msgs: Vec<Msg>,
    },

    #[serde(rename = "staking/MsgDelegate")]
    MsgDelegate {
        delegator_address: Option<Address>,
        validator_address: Option<Address>,
        amount: Amount,
    },

    #[serde(rename = "staking/MsgBeginRedelegate")]
    MsgBeginRedelegate {
        delegator_address: Option<Address>,
        validator_src_address: Option<Address>,
        validator_dst_address: Option<Address>,
        amount: Amount,
    },

    #[serde(rename = "staking/MsgUndelegate")]
    MsgUndelegate {
        delegator_address: Option<Address>,
        validator_address: Option<Address>,
        amount: Amount,
    },

    #[serde(rename = "gov/MsgVote")]
    MsgVote {
        proposal_id: Id,
        voter: Option<Address>,
        option: Option<String>,
    },

    #[serde(rename = "oracle/MsgAggregateExchangeRatePrevote")]
    MsgAggregateExchangeRatePrevote {
        hash: Option<Hash>,
        feeder: Option<Address>,
        validator: Option<String>,
        sender: Option<Address>,
        contract: Option<Address>,
        execute_msg: Option<String>,
        #[serde(default)]
        coins: Vec<Coin>,
        trader: Option<Address>,
        offer_coin: Option<Coin>,
        ask_denom: Option<String>,
        #[serde(default)]
        inputs: Vec<IO>,
        #[serde(default)]
        outputs: Vec<IO>,
    },

    #[serde(rename = "oracle/MsgAggregateExchangeRateVote")]
    MsgAggregateExchangeRateVote {
        salt: Option<String>,
        exchange_rates: Option<String>,
        feeder: Option<Address>,
        validator: Option<String>,
    },

    #[serde(rename = "wasm/MsgExecuteContract")]
    MsgExecuteContract {
        sender: Option<Address>,
        contract: Option<Address>,
        execute_msg: Option<String>,
        #[serde(default)]
        coins: Vec<Coin>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Amounts {
    Single(Amount),
    Multi(Vec<Amount>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IO {
    pub address: Option<String>,
    #[serde(default)]
    pub coins: Vec<Coin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub denom: Option<String>,
    pub amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Amount {
    pub denom: Option<String>,
    pub amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StdFee {
    pub amount: Option<Amounts>,
    pub gas: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StdSignature {
    pub pub_key: PubKey,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubKeyWrapper {
    #[serde(rename = "Sum")]
    pub sum: PubKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum PubKey {
    #[serde(rename = "tendermint/PubKeySecp256k1")]
    PubKeySecp256k1 { value: Option<String> },
    #[serde(rename = "tendermint.crypto.PublicKey_Ed25519")]
    Ed25519 { ed25519: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PubKeyType {
    Ed25519,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;

    /// Helper function for reading a file to string.
    fn read_file<P>(path: P) -> io::Result<String>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }

    #[test]
    fn can_deserialize_serialize_new_block_4737552() {
        let json = read_file("tests/fixtures/new_block_4737552.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_4739621() {
        let json = read_file("tests/fixtures/new_block_4739621.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_473729() {
        let json = read_file("tests/fixtures/new_block_4739729.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_4739763() {
        let json = read_file("tests/fixtures/new_block_4739763.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_4739836() {
        let json = read_file("tests/fixtures/new_block_4739836.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_4739887() {
        let json = read_file("tests/fixtures/new_block_4739887.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }

    #[test]
    fn can_deserialize_serialize_new_block_4739954() {
        let json = read_file("tests/fixtures/new_block_4739954.json")
            .expect("Could not read JSON fixture");
        let de: EventType =
            serde_json::from_str(&json).expect("Could not deserialize JSON fixture");
        serde_json::to_string(&de).expect("Could not serialize JSON");
    }
}
