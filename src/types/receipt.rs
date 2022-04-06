use crate::types::log::TransactionLog;
use ethereum_types::{Address, Bloom, H256, U64};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Transaction receipt.
pub struct TransactionReceipt {
    /// 256-bit transaction hash.
    pub transaction_hash: H256,
    /// Transaction index.
    pub transaction_index: U64,
    /// Block hash.
    pub block_hash: H256,
    /// Block number.
    pub block_number: U64,
    /// Transaction sender.
    pub from: Address,
    /// Transaction recipient.
    pub to: Option<Address>,
    /// Cumulative gas used at the moment this transaction was included.
    pub cumulative_gas_used: U64,
    /// Gas used in this transaction.
    pub gas_used: U64,
    /// Contract address if transaction creates contract.
    pub contract_address: Option<Address>,
    /// Logs generated by this transaction.
    pub logs: Vec<TransactionLog>,
    /// Bloom logs.
    pub logs_bloom: Bloom,
    /// Status, 1 for success, 0 for failure.
    pub status: U64,
}
