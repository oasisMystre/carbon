use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LbPairCreate {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_step: u16,
    pub token_x: solana_pubkey::Pubkey,
    pub token_y: solana_pubkey::Pubkey,
}
