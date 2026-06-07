use solana_pubkey::Pubkey;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd399583e953cb146")]
pub struct InitializeRewardEvent {
    pub lb_pair: Pubkey,
    pub reward_mint: Pubkey,
    pub funder: Pubkey,
    pub reward_index: u64,
    pub reward_duration: u64,
}
