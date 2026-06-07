use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeTier {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
