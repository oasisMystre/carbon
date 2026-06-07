use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01ee058ecf3e2ce4")]
pub struct LimitOrderState {
    pub pool_id: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub tick_index: i32,
    pub zero_for_one: bool,
    pub order_phase: u64,
    pub total_amount: u64,
    pub filled_amount: u64,
    pub settle_base: u64,
    pub settled_output: u64,
    pub open_time: u64,
    pub unfilled_ratio_x64: u128,
    pub padding: [u64; 4],
}
