use crate::prelude::*;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use std::collections::HashSet;

#[rpc(server, client, namespace = "trace")]
pub trait TraceApi {
    /// Ad-hoc part of the tracing API.
    /// Executes the given call and returns a number of possible traces for it.
    #[method(name = "call")]
    async fn call(
        &self,
        call: MessageCall,
        trace_type: HashSet<TraceType>,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<BlockTrace>;
    /// Performs multiple call traces on top of the same block.
    /// Transaction n will be executed on top of a pending block with all n-1 transactions applied (traced) first.
    /// Allows to trace dependent transactions.
    #[method(name = "callMany")]
    async fn call_many(
        &self,
        calls: Vec<(MessageCall, HashSet<TraceType>)>,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<Vec<BlockTrace>>;
    /// Traces a call to eth_sendRawTransaction without making the call, returning the traces
    #[method(name = "rawTransaction")]
    async fn raw_transaction(
        &self,
        rlp: Bytes,
        trace_type: HashSet<TraceType>,
    ) -> RpcResult<BlockTrace>;
    /// Replays all transactions in a block returning the requested traces for each transaction.
    #[method(name = "replayBlockTransactions")]
    async fn replay_block_transactions(
        &self,
        block_number: BlockNumber,
        trace_type: HashSet<TraceType>,
    ) -> RpcResult<Vec<BlockTrace>>;
    /// Replays a transaction, returning the traces.
    #[method(name = "replayTransaction")]
    async fn replay_transaction(
        &self,
        hash: H256,
        trace_type: HashSet<TraceType>,
    ) -> RpcResult<BlockTrace>;
    /// Returns traces created at given block.
    #[method(name = "block")]
    async fn block(&self, block_number: BlockNumber) -> RpcResult<Vec<BlockTrace>>;

    /// Returns traces matching given filter.
    #[method(name = "filter")]
    async fn filter(
        &self,
        from_block: Option<BlockNumber>,
        to_block: Option<BlockNumber>,
        from_address: Option<HashSet<Address>>,
        to_address: Option<HashSet<Address>>,
    ) -> RpcResult<Vec<BlockTrace>>;
}
