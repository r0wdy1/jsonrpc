mod debug;
mod erigon;
mod eth;
mod net;
mod otterscan;
mod trace;
pub mod types;

pub use debug::*;
pub use erigon::*;
pub use eth::*;
pub use net::*;
pub use otterscan::*;
pub use trace::*;

mod prelude {
    pub use crate::types::*;
    pub use ethereum_types::{Address, Bloom, H256, H64, U64};
    pub use ethnum::U256;
    pub use std::collections::HashSet;

    pub use jsonrpsee::{core::RpcResult, proc_macros::rpc};
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
}
