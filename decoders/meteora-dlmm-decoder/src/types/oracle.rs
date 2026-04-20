

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Oracle {
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
}
