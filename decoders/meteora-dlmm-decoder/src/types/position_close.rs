use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionClose {
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
