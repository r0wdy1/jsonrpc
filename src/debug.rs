use crate::prelude::*;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AccountRangeResult {}

#[rpc(server, client, namespace = "debug")]
pub trait DebugApi {
    #[method(name = "accountRange")]
    async fn account_range(
        &self,
        block_id: BlockId,
        index: u64,
        start: H256,
        limit: u64,
    ) -> RpcResult<()>;

    #[method(name = "getModifiedAccountsByNumber")]
    async fn get_modified_accounts_by_number(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<Vec<Address>>;
    #[method(name = "getModifiedAccountsByHash")]
    async fn get_modified_accounts_by_hash(&self, block_hash: H256) -> RpcResult<Vec<Address>>;
    #[method(name = "traceTransaction")]
    async fn trace_transaction(&self, transaction_hash: H256) -> RpcResult<()>;
}
