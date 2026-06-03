

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdateRewardDuration {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}
