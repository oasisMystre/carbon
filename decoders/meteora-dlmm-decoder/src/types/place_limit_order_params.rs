use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlaceLimitOrderParams {
    pub is_ask_side: bool,
    pub padding: [u8; 16],
    pub relative_bin: Option<RelativeBin>,
    pub bins: Vec<BinLimitOrderAmount>,
}
