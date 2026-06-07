use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x759d3c674231a300")]
pub struct DecreaseLimitOrder {
    pub amount: u64,
    pub amount_min: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecreaseLimitOrderInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub tick_array: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub output_vault: solana_pubkey::Pubkey,
    pub input_vault_mint: solana_pubkey::Pubkey,
    pub output_vault_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLimitOrder {
    type ArrangedAccounts = DecreaseLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let tick_array = next_account(&mut iter)?;
        let limit_order = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let input_vault = next_account(&mut iter)?;
        let output_vault = next_account(&mut iter)?;
        let input_vault_mint = next_account(&mut iter)?;
        let output_vault_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;

        Some(DecreaseLimitOrderInstructionAccounts {
            owner,
            pool_state,
            tick_array,
            limit_order,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            input_vault_mint,
            output_vault_mint,
            token_program,
            token_program_2022,
        })
    }
}
