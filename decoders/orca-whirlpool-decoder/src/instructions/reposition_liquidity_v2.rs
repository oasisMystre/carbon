use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbfa9e00b83139efd")]
pub struct RepositionLiquidityV2 {
    pub new_tick_lower_index: i32,
    pub new_tick_upper_index: i32,
    pub method: RepositionLiquidityMethod,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RepositionLiquidityV2InstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub existing_tick_array_lower: solana_pubkey::Pubkey,
    pub existing_tick_array_upper: solana_pubkey::Pubkey,
    pub new_tick_array_lower: solana_pubkey::Pubkey,
    pub new_tick_array_upper: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepositionLiquidityV2 {
    type ArrangedAccounts = RepositionLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let token_mint_a = next_account(&mut iter)?;
        let token_mint_b = next_account(&mut iter)?;
        let token_owner_account_a = next_account(&mut iter)?;
        let token_owner_account_b = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let existing_tick_array_lower = next_account(&mut iter)?;
        let existing_tick_array_upper = next_account(&mut iter)?;
        let new_tick_array_lower = next_account(&mut iter)?;
        let new_tick_array_upper = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RepositionLiquidityV2InstructionAccounts {
            whirlpool,
            token_program_a,
            token_program_b,
            memo_program,
            position_authority,
            funder,
            position,
            position_token_account,
            token_mint_a,
            token_mint_b,
            token_owner_account_a,
            token_owner_account_b,
            token_vault_a,
            token_vault_b,
            existing_tick_array_lower,
            existing_tick_array_upper,
            new_tick_array_lower,
            new_tick_array_upper,
            system_program,
        })
    }
}
