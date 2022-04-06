use crate::prelude::*;
use crate::types::*;

#[rpc(server, client, namespace = "eth")]
pub trait EthApi {
    #[method(name = "blockNumber")]
    async fn block_number(&self) -> RpcResult<U64>;
    #[method(name = "chainId")]
    async fn chain_id(&self) -> RpcResult<U64>;
    #[method(name = "call")]
    async fn call(&self, call_data: MessageCall, block_number: BlockNumber) -> RpcResult<Bytes>;
    #[method(name = "estimateGas")]
    async fn estimate_gas(
        &self,
        call_data: MessageCall,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "getBalance")]
    async fn get_balance(&self, address: Address, block_number: BlockNumber) -> RpcResult<U256>;
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
    async fn get_block_transaction_count_by_hash(&self, block_hash: H256) -> RpcResult<U64>;
    #[method(name = "getBlockTransactionCountByNumber")]
    async fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "getCode")]
    async fn get_code(&self, address: Address, block_number: BlockNumber) -> RpcResult<Bytes>;
    #[method(name = "getStorageAt")]
    async fn get_storage_at(
        &self,
        address: Address,
        storage_pos: U256,
        block_number: BlockNumber,
    ) -> RpcResult<U256>; // Storage data is nothing more than 32-bytes
    #[method(name = "getTransactionByHash")]
    async fn get_transaction_by_hash(&self, hash: H256) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockHashAndIndex")]
    async fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: U64,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockNumberAndIndex")]
    async fn get_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: U64,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionCount")]
    async fn get_transaction_count(
        &self,
        address: Address,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "getTransactionReceipt")]
    async fn get_transaction_receipt(&self, tx_hash: H256)
        -> RpcResult<Option<TransactionReceipt>>;
    #[method(name = "getUncleByBlockHashAndIndex")]
    async fn get_uncle_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: U64,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleByBlockNumberAndIndex")]
    async fn get_uncle_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: U64,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleCountByBlockHash")]
    async fn get_uncle_count_by_block_hash(&self, block_hash: H256) -> RpcResult<U64>;
    #[method(name = "getUncleCountByBlockNumber")]
    async fn get_uncle_count_by_block_number(&self, block_number: BlockNumber) -> RpcResult<U64>;
}
