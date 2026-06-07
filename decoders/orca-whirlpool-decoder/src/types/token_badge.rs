use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenBadge {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub attribute_require_non_transferable_position: bool,
}
