use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateRewardFunder {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub old_funder: solana_pubkey::Pubkey,
    pub new_funder: solana_pubkey::Pubkey,
}
