use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e2729876fb7c840")]
pub struct InitializeCustomizablePermissionlessLbPair {
    pub params: CustomizableParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeCustomizablePermissionlessLbPairInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub token_mint_x: solana_pubkey::Pubkey,
    pub token_mint_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCustomizablePermissionlessLbPair {
    type ArrangedAccounts = InitializeCustomizablePermissionlessLbPairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let token_mint_x = next_account(&mut iter)?;
        let token_mint_y = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let user_token_x = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let user_token_y = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(
            InitializeCustomizablePermissionlessLbPairInstructionAccounts {
                lb_pair,
                bin_array_bitmap_extension,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                user_token_x,
                funder,
                token_program,
                system_program,
                user_token_y,
                event_authority,
                program,
            },
        )
    }
}
