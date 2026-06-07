use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcd4e74215c691a60")]
pub struct SettleLimitOrder {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SettleLimitOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub tick_array: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub output_vault: solana_pubkey::Pubkey,
    pub output_vault_mint: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleLimitOrder {
    type ArrangedAccounts = SettleLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let tick_array = next_account(&mut iter)?;
        let limit_order = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let output_vault = next_account(&mut iter)?;
        let output_vault_mint = next_account(&mut iter)?;
        let output_token_program = next_account(&mut iter)?;

        Some(SettleLimitOrderInstructionAccounts {
            signer,
            pool_state,
            tick_array,
            limit_order,
            output_token_account,
            output_vault,
            output_vault_mint,
            output_token_program,
        })
    }
}
