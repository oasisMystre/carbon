use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5f82b584fb32c326")]
pub struct LiquidityRepositionedEvent {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub existing_range_tick_lower_index: i32,
    pub existing_range_tick_upper_index: i32,
    pub new_range_tick_lower_index: i32,
    pub new_range_tick_upper_index: i32,
    pub existing_range_liquidity: u128,
    pub new_range_liquidity: u128,
    pub existing_range_token_a_amount: u64,
    pub existing_range_token_b_amount: u64,
    pub new_range_token_a_amount: u64,
    pub new_range_token_b_amount: u64,
    pub token_a_transfer_amount: u64,
    pub token_a_transfer_fee: u64,
    pub is_token_a_transfer_from_owner: bool,
    pub token_b_transfer_amount: u64,
    pub token_b_transfer_fee: u64,
    pub is_token_b_transfer_from_owner: bool,
}
