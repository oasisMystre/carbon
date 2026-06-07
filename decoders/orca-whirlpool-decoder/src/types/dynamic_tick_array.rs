use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DynamicTickArray {
    pub start_tick_index: i32,
    pub whirlpool: solana_pubkey::Pubkey,
    pub tick_bitmap: u128,
    #[serde(with = "serde_big_array::BigArray")]
    pub ticks: [DynamicTick; 88],
}
