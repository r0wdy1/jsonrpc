use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
/// Wrapper over bytes::Bytes that implements serde::Serialize and serde::Deserialize.
pub struct Bytes(pub bytes::Bytes);

impl Default for Bytes {
    fn default() -> Self {
        Bytes(bytes::Bytes::new())
    }
}

impl ToString for Bytes {
    fn to_string(&self) -> String {
        format!("0x{}", hex::encode(self.0.as_ref()))
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl From<&'static [u8]> for Bytes {
    fn from(bytes: &'static [u8]) -> Self {
        Bytes(bytes::Bytes::from(bytes))
    }
}

impl From<bytes::Bytes> for Bytes {
    fn from(v: bytes::Bytes) -> Self {
        Bytes(v)
    }
}

impl From<Bytes> for bytes::Bytes {
    fn from(v: Bytes) -> Self {
        v.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes(v.into())
    }
}

impl From<U256> for Bytes {
    fn from(v: U256) -> Self {
        Bytes(v.to_be_bytes().to_vec().into())
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(hex::decode(s.strip_prefix("0x").unwrap_or(&s))
            .map_err(serde::de::Error::custom)?
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_serialize() {
        let bytes = Bytes(vec![1, 2, 3].into());
        assert_eq!(bytes.to_string(), "0x010203");
    }
}
