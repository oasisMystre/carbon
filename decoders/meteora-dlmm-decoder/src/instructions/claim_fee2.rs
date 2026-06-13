use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x70bf65ab1c907fbb")]
pub struct ClaimFee2 {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimFee2InstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub token_program_x: solana_pubkey::Pubkey,
    pub token_program_y: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFee2 {
    type ArrangedAccounts = ClaimFee2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let user_token_x = next_account(&mut iter)?;
        let user_token_y = next_account(&mut iter)?;
        let token_x_mint = next_account(&mut iter)?;
        let token_y_mint = next_account(&mut iter)?;
        let token_program_x = next_account(&mut iter)?;
        let token_program_y = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ClaimFee2InstructionAccounts {
            lb_pair,
            position,
            sender,
            reserve_x,
            reserve_y,
            user_token_x,
            user_token_y,
            token_x_mint,
            token_y_mint,
            token_program_x,
            token_program_y,
            memo_program,
            event_authority,
            program,
        })
    }
}
