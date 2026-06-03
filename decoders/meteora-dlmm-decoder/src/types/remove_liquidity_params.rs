

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RemoveLiquidityParams {
    pub min_bin_id: Option<i32>,
    pub max_bin_id: Option<i32>,
    pub bps: u16,
    pub padding: [u8; 16],
}
