use solana_pubkey::Pubkey;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d006d75b33d5bc7c8")]
pub struct RebalancingEvent {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub active_bin_id: i32,
    pub x_withdrawn_amount: u64,
    pub x_added_amount: u64,
    pub y_withdrawn_amount: u64,
    pub y_added_amount: u64,
    pub x_fee_amount: u64,
    pub y_fee_amount: u64,
    pub old_min_id: i32,
    pub old_max_id: i32,
    pub new_min_id: i32,
    pub new_max_id: i32,
    pub rewards: [u64; 2],
}
