
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RebalanceLiquidityParams {
    pub active_id: i32,
    pub max_active_bin_slippage: u16,
    pub should_claim_fee: bool,
    pub should_claim_reward: bool,
    pub min_withdraw_x_amount: u64,
    pub max_deposit_x_amount: u64,
    pub min_withdraw_y_amount: u64,
    pub max_deposit_y_amount: u64,
    pub shrink_mode: u8,
    pub padding: [u8; 31],
    pub removes: Vec<RemoveLiquidityParams>,
    pub adds: Vec<AddLiquidityParams>,
}
