use solana_pubkey::Pubkey;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de7bd419566d79af4")]
pub struct WithdrawIneligibleRewardEvent {
    pub lb_pair: Pubkey,
    pub reward_mint: Pubkey,
    pub amount: u64,
}
