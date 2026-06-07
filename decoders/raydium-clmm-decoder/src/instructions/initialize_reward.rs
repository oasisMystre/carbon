use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub param: InitializeRewardParam,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardInstructionAccounts {
    pub reward_funder: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub operation_state: solana_pubkey::Pubkey,
    pub reward_token_mint: solana_pubkey::Pubkey,
    pub reward_token_vault: solana_pubkey::Pubkey,
    pub reward_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let reward_funder = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let operation_state = next_account(&mut iter)?;
        let reward_token_mint = next_account(&mut iter)?;
        let reward_token_vault = next_account(&mut iter)?;
        let reward_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(InitializeRewardInstructionAccounts {
            reward_funder,
            funder_token_account,
            amm_config,
            pool_state,
            operation_state,
            reward_token_mint,
            reward_token_vault,
            reward_token_program,
            system_program,
            rent,
        })
    }
}
