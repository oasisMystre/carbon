use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DecreaseLimitOrderEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub zero_for_one: bool,
    pub tick_index: i32,
    pub total_amount: u64,
    pub filled_amount: u64,
    pub settled_output_amount: u64,
    pub decreased_amount: u64,
}
