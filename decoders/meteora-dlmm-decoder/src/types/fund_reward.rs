use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FundReward {
    pub lb_pair: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub amount: u64,
}
