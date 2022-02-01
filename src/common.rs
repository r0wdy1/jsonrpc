use crate::prelude::*;

pub type BlockNumber = U64;
pub type EstimateGas = U64;
pub type Balance = U256;
pub type TxCount = U64;
pub type StoragePos = U256;
pub type StorageData = U256;
pub type TxIndex = U64;
pub type UncleIndex = U64;
pub type UncleCount = U64;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallData {
    pub from: Option<Address>,
    pub to: Address,
    pub gas: Option<U64>,
    pub gas_price: Option<U64>,
    pub value: Option<U256>,
    #[serde(with = "hexbytes_option")]
    pub data: Option<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnData(#[serde(with = "hexbytes")] pub Bytes);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Code(#[serde(with = "hexbytes")] pub Bytes);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: Option<U64>,
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub nonce: Option<H64>,
    pub sha3_uncles: H256,
    pub logs_bloom: Option<Bloom>,
    pub transactions_root: H256,
    pub state_root: H256,
    pub receipts_root: H256,
    pub miner: Address,
    pub difficulty: U256,
    pub total_difficulty: U256,
    #[serde(with = "hexbytes")]
    pub extra_data: Bytes,
    pub size: U64,
    pub gas_limit: U64,
    pub gas_used: U64,
    pub timestamp: U64,
    pub transactions: Vec<Transaction>,
    pub uncles: Vec<H256>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tx {
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub from: Address,
    pub gas: U64,
    pub gas_price: U256,
    pub hash: H256,
    #[serde(with = "hexbytes")]
    pub input: Bytes,
    pub nonce: U64,
    pub to: Option<Address>,
    pub transaction_index: Option<U64>,
    pub value: U256,
    pub v: U64,
    pub r: H256,
    pub s: H256,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Transaction {
    Partial(H256),
    Full(Box<Tx>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxLog {
    pub log_index: Option<U64>,
    pub transaction_index: Option<U64>,
    pub transaction_hash: Option<H256>,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub address: Address,
    #[serde(with = "hexbytes")]
    pub data: Bytes,
    pub topics: Vec<H256>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxReceipt {
    pub transaction_hash: H256,
    pub transaction_index: U64,
    pub block_hash: H256,
    pub block_number: U64,
    pub from: Address,
    pub to: Option<Address>,
    pub cumulative_gas_used: U64,
    pub gas_used: U64,
    pub contract_address: Option<Address>,
    pub logs: Vec<TxLog>,
    pub logs_bloom: Bloom,
    pub status: U64,
}

pub mod hexbytes {
    use serde::Serializer;

    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(hex::decode(s.strip_prefix("0x").unwrap_or(&s))
            .map_err(D::Error::custom)?
            .into())
    }

    pub fn serialize<S>(b: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(b)))
    }
}

pub mod hexbytes_option {
    use serde::Serializer;

    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Bytes>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer).and_then(|o| {
            Ok(match o {
                None => None,
                Some(s) => Some(
                    hex::decode(s.strip_prefix("0x").unwrap_or(&s))
                        .map_err(D::Error::custom)?
                        .into(),
                ),
            })
        })
    }

    pub fn serialize<S>(o: &Option<Bytes>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match o {
            Some(b) => serializer.serialize_str(&format!("0x{}", hex::encode(b))),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_de_hexbytes() {
        let code = Code(bytes::Bytes::from(&b"Hello Akula"[..]));
        let hexstring = r#""0x48656c6c6f20416b756c61""#;
        assert_eq!(serde_json::to_string(&code).unwrap(), hexstring);
        assert_eq!(serde_json::from_str::<Code>(hexstring).unwrap(), code);
    }

    #[test]
    fn test_ser_de_hexbytes_option() {
        let call_data = CallData {
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
            serde_json::from_str::<CallData>(hexstring).unwrap(),
            call_data
        );

        let call_data_with_data = CallData {
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
            serde_json::from_str::<CallData>(hexstring_with_data).unwrap(),
            call_data_with_data
        );
    }
}
