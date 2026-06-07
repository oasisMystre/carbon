use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WhirlpoolsConfigExtension {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub config_extension_authority: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
}
