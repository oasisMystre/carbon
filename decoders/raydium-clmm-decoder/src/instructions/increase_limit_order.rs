use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb19059ecfaba7d63")]
pub struct IncreaseLimitOrder {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IncreaseLimitOrderInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub tick_array: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub input_vault_mint: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseLimitOrder {
    type ArrangedAccounts = IncreaseLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let tick_array = next_account(&mut iter)?;
        let limit_order = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let input_vault = next_account(&mut iter)?;
        let input_vault_mint = next_account(&mut iter)?;
        let input_token_program = next_account(&mut iter)?;

        Some(IncreaseLimitOrderInstructionAccounts {
            owner,
            pool_state,
            tick_array,
            limit_order,
            input_token_account,
            input_vault,
            input_vault_mint,
            input_token_program,
        })
    }
}
