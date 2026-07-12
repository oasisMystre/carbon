use {
    carbon_core::{borsh, CarbonDeserialize},
    solana_pubkey::Pubkey,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x40c6cde8260871e2")]
pub struct SwapEvent {
    pub pool_state: Pubkey,
    pub sender: Pubkey,
    pub token_account0: Pubkey,
    pub token_account1: Pubkey,
    pub amount0: u64,
    pub transfer_fee0: u64,
    pub amount1: u64,
    pub transfer_fee1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
    pub trade_fee0: u64,
    pub trade_fee1: u64,
}
