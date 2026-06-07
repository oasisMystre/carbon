use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WhirlpoolRewardInfo {
    pub mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub extension: [u8; 32],
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}
