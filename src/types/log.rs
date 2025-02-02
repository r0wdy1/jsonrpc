use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Transaction's log entry.
pub struct TransactionLog {
    /// Log's index within transaction.
    pub log_index: Option<U64>,
    /// Transaction's index within block.
    pub transaction_index: Option<U64>,
    /// Transaction's hash.
    pub transaction_hash: Option<H256>,
    /// Block's hash, transaction is included in.
    pub block_hash: Option<H256>,
    /// Block number, transaction is included in.
    pub block_number: Option<U64>,
    /// Log's address.
    pub address: Address,
    /// Log's data.
    pub data: Bytes,
    /// Log's Topics.
    pub topics: Vec<H256>,
}
