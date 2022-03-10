use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
/// MessageCall, used with `eth_call`.
pub struct MessageCall {
    /// Sender, defaults to '0x0000000000000000000000000000000000000000'.
    pub from: Option<Address>,
    /// Recipient.
    pub to: Address,
    /// Gas limit.
    pub gas: Option<U64>,
    /// Gas price.
    pub gas_price: Option<U256>,
    /// Value to transfer.
    pub value: Option<U256>,
    /// Data to send.
    pub data: Option<Bytes>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// A transaction hash.
    pub hash: H256,
    /// Transactions's nonce.
    pub nonce: U64,
    /// Block hash.
    pub block_hash: Option<H256>,
    /// Block number.
    pub block_number: Option<U64>,
    // Transaction sender.
    pub from: Address,
    /// Transaction's gas limit.
    pub gas: U64,
    /// Transaction's gas price.
    pub gas_price: U256,
    /// Transaction's data.
    pub input: Bytes,
    /// Transaction's recipient (None when contract creation).
    pub to: Option<Address>,
    /// Index inside block.
    pub transaction_index: Option<U64>,
    /// Transaction's value.
    pub value: U256,
    /// RLP encoded representation of the transaction.
    pub v: U64,
    pub r: H256,
    pub s: H256,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Tx is either a transaction or a transaction hash.
pub enum Tx {
    /// Transaction.
    Transaction(Box<Transaction>),
    /// Transaction hash.
    Hash(H256),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_de_hexbytes_option() {
        let call_data = MessageCall {
            from: None,
            to: Address::from([0; 20]),
            gas: None,
            gas_price: None,
            value: None,
            data: None,
        };
        let hexstring = r#"{"from":null,"to":"0x0000000000000000000000000000000000000000","gas":null,"gasPrice":null,"value":null,"data":null}"#;
        assert_eq!(serde_json::to_string(&call_data).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_str::<MessageCall>(hexstring).unwrap(),
            call_data
        );

        let call_data_with_data = MessageCall {
            from: None,
            to: Address::from([0; 20]),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes::from(&b"Hello Akula"[..])),
        };

        let hexstring_with_data = r#"{"from":null,"to":"0x0000000000000000000000000000000000000000","gas":null,"gasPrice":null,"value":null,"data":"0x48656c6c6f20416b756c61"}"#;
        assert_eq!(
            serde_json::to_string(&call_data_with_data).unwrap(),
            hexstring_with_data
        );
        assert_eq!(
            serde_json::from_str::<MessageCall>(hexstring_with_data).unwrap(),
            call_data_with_data
        );
    }
}
