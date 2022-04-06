use crate::prelude::*;

#[rpc(server, client, namespace = "erigon")]
pub trait ErigonApi {
    #[method(name = "getHeaderByNumber")]
    async fn get_header_by_number(&self, block_number: u64) -> RpcResult<Option<Header>>;
}
