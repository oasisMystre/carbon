use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DecreasePositionLength {
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub length_to_remove: u16,
    pub side: u8,
}
