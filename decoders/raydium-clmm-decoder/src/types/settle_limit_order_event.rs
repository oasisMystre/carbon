use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SettleLimitOrderEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub zero_for_one: bool,
    pub tick_index: i32,
    pub total_amount: u64,
    pub filled_amount: u64,
    pub settled_amount_out: u64,
}
