use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12eda6c52210d590")]
pub struct CollectRemainingRewards {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectRemainingRewardsInstructionAccounts {
    pub reward_funder: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub reward_token_vault: solana_pubkey::Pubkey,
    pub reward_vault_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectRemainingRewards {
    type ArrangedAccounts = CollectRemainingRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let reward_funder = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let reward_token_vault = next_account(&mut iter)?;
        let reward_vault_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;

        Some(CollectRemainingRewardsInstructionAccounts {
            reward_funder,
            funder_token_account,
            pool_state,
            reward_token_vault,
            reward_vault_mint,
            token_program,
            token_program_2022,
            memo_program,
        })
    }
}
