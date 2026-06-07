use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TickState {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,
    pub reward_growths_outside_x64: [u128; 3],
    pub order_phase: u64,
    pub orders_amount: u64,
    pub part_filled_orders_remaining: u64,
    pub unfilled_ratio_x64: u128,
    pub padding: [u32; 3],
}
