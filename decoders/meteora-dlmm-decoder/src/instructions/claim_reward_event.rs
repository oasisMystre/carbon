use solana_pubkey::Pubkey;


use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d947486cc16ab555f")]
pub struct ClaimRewardEvent{
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub reward_index: u64,
    pub total_reward: u64,
}
