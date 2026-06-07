use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b44d4a7592fa401")]
pub struct CreateCustomizablePool {
    pub customizable_params: CreateCustomizableParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCustomizablePoolInstructionAccounts {
    pub pool_creator: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_mint_0: solana_pubkey::Pubkey,
    pub token_mint_1: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub tick_array_bitmap: solana_pubkey::Pubkey,
    pub token_program_0: solana_pubkey::Pubkey,
    pub token_program_1: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCustomizablePool {
    type ArrangedAccounts = CreateCustomizablePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_creator = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let token_mint_0 = next_account(&mut iter)?;
        let token_mint_1 = next_account(&mut iter)?;
        let token_vault_0 = next_account(&mut iter)?;
        let token_vault_1 = next_account(&mut iter)?;
        let observation_state = next_account(&mut iter)?;
        let tick_array_bitmap = next_account(&mut iter)?;
        let token_program_0 = next_account(&mut iter)?;
        let token_program_1 = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(CreateCustomizablePoolInstructionAccounts {
            pool_creator,
            amm_config,
            pool_state,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_state,
            tick_array_bitmap,
            token_program_0,
            token_program_1,
            system_program,
            rent,
        })
    }
}
