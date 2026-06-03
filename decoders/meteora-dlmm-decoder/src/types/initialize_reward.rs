

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct InitializeReward {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub reward_duration: u64,
}
