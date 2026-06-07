use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TickArray {
    pub start_tick_index: i32,
    #[serde(with = "serde_big_array::BigArray")]
    pub ticks: [Tick; 88],
    pub whirlpool: solana_pubkey::Pubkey,
}
