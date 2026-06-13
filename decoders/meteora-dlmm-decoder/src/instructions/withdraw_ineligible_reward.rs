use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x94ce2ac3f7316708")]
pub struct WithdrawIneligibleReward {
    pub reward_index: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawIneligibleRewardInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub bin_array: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawIneligibleReward {
    type ArrangedAccounts = WithdrawIneligibleRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let bin_array = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(WithdrawIneligibleRewardInstructionAccounts {
            lb_pair,
            reward_vault,
            reward_mint,
            funder_token_account,
            funder,
            bin_array,
            token_program,
            memo_program,
            event_authority,
            program,
        })
    }
}
