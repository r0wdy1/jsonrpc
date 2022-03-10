use crate::types::{BlockNumber, BlockTrace, TraceType, Tx};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server, client, namespace = "trace")]
pub trait TraceApi {
    #[method(name = "call")]
    async fn call(
        &self,
        call: Tx,
        trace_type: Vec<TraceType>,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<BlockTrace>;
}
