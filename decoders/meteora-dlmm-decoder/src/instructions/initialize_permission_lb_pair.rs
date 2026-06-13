use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c66d555fb033515")]
pub struct InitializePermissionLbPair {
    pub ix_data: InitPermissionPairIx,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePermissionLbPairInstructionAccounts {
    pub base: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub token_mint_x: solana_pubkey::Pubkey,
    pub token_mint_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub token_badge_x: solana_pubkey::Pubkey,
    pub token_badge_y: solana_pubkey::Pubkey,
    pub token_program_x: solana_pubkey::Pubkey,
    pub token_program_y: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePermissionLbPair {
    type ArrangedAccounts = InitializePermissionLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let base = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let token_mint_x = next_account(&mut iter)?;
        let token_mint_y = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let token_badge_x = next_account(&mut iter)?;
        let token_badge_y = next_account(&mut iter)?;
        let token_program_x = next_account(&mut iter)?;
        let token_program_y = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializePermissionLbPairInstructionAccounts {
            base,
            lb_pair,
            bin_array_bitmap_extension,
            token_mint_x,
            token_mint_y,
            reserve_x,
            reserve_y,
            oracle,
            payer,
            operator,
            signer,
            token_badge_x,
            token_badge_y,
            token_program_x,
            token_program_y,
            system_program,
            event_authority,
            program,
        })
    }
}
