use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x849c841f4328e861")]
pub struct CancelLimitOrder {
    pub bins: Vec<i32>,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelLimitOrderInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub owner_token_x: solana_pubkey::Pubkey,
    pub owner_token_y: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelLimitOrder {
    type ArrangedAccounts = CancelLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let token_x_mint = next_account(&mut iter)?;
        let token_y_mint = next_account(&mut iter)?;
        let limit_order = next_account(&mut iter)?;
        let owner_token_x = next_account(&mut iter)?;
        let owner_token_y = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_x_program = next_account(&mut iter)?;
        let token_y_program = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CancelLimitOrderInstructionAccounts {
            lb_pair,
            bin_array_bitmap_extension,
            reserve_x,
            reserve_y,
            token_x_mint,
            token_y_mint,
            limit_order,
            owner_token_x,
            owner_token_y,
            owner,
            token_x_program,
            token_y_program,
            memo_program,
            event_authority,
            program,
        })
    }
}
