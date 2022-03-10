use crate::prelude::*;

#[rpc(server, client, namespace = "net")]
pub trait NetApi {
    #[method(name = "listening")]
    async fn listening(&self) -> RpcResult<bool>;
    #[method(name = "peerCount")]
    async fn peer_count(&self) -> RpcResult<U64>;
    #[method(name = "version")]
    async fn version(&self) -> RpcResult<U64>;
}
