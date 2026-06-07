use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d0b780dccc75713c8")]
pub struct IncreaseLimitOrderEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub zero_for_one: bool,
    pub tick_index: i32,
    pub total_amount: u64,
    pub increased_amount: u64,
    pub transfer_fee: u64,
}
