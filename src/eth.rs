use crate::common::*;
use crate::prelude::*;

#[rpc(server, client, namespace = "eth")]
pub trait EthApi {
    #[method(name = "blockNumber")]
    async fn block_number(&self) -> RpcResult<BlockNumber>;
    #[method(name = "call")]
    async fn call(&self, call_data: CallData, block_number: BlockNumber) -> RpcResult<ReturnData>;
    #[method(name = "estimateGas")]
    async fn estimate_gas(
        &self,
        call_data: CallData,
        block_number: BlockNumber,
    ) -> RpcResult<EstimateGas>;
    #[method(name = "getBalance")]
    async fn get_balance(&self, address: Address, block_number: BlockNumber) -> RpcResult<Balance>;
    #[method(name = "getBlockByHash")]
    async fn get_block_by_hash(
        &self,
        block_hash: H256,
        full_tx_obj: bool,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getBlockByNumber")]
    async fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_tx_obj: bool,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getBlockTransactionCountByHash")]
    async fn get_block_tx_count_by_hash(&self, block_hash: H256) -> RpcResult<TxCount>;
    #[method(name = "getBlockTransactionCountByNumber")]
    async fn get_block_tx_count_by_number(&self, block_number: BlockNumber) -> RpcResult<TxCount>;
    #[method(name = "getCode")]
    async fn get_code(&self, address: Address, block_number: BlockNumber) -> RpcResult<Code>;
    #[method(name = "getStorageAt")]
    async fn get_storage_at(
        &self,
        address: Address,
        storage_pos: StoragePos,
        block_number: BlockNumber,
    ) -> RpcResult<StorageData>;
    #[method(name = "getTransaction")]
    async fn get_transaction(&self, hash: H256) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockHashAndIndex")]
    async fn get_tx_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: TxIndex,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockNumberAndIndex")]
    async fn get_tx_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: TxIndex,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionCount")]
    async fn get_transaction_count(
        &self,
        address: Address,
        block_number: BlockNumber,
    ) -> RpcResult<TxCount>;
    #[method(name = "getTransactionReceipt")]
    async fn get_transaction_receipt(&self, tx_hash: H256) -> RpcResult<Option<TxReceipt>>;
    #[method(name = "getUncleByBlockHashAndIndex")]
    async fn get_uncle_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: UncleIndex,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleByBlockNumberAndIndex")]
    async fn get_uncle_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: UncleIndex,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleCountByBlockHash")]
    async fn get_uncle_count_by_block_hash(&self, block_hash: H256) -> RpcResult<UncleCount>;
    #[method(name = "getUncleCountByBlockNumber")]
    async fn get_uncle_count_by_block_number(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<UncleCount>;
}
