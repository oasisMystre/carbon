use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ClaimFee {
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub fee_x: u64,
    pub fee_y: u64,
}
