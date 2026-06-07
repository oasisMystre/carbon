use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionBundle {
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bitmap: [u8; 32],
}
