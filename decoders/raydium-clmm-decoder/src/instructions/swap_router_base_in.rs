use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x457d73daf5baf2c4")]
pub struct SwapRouterBaseIn {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapRouterBaseInInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub input_token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapRouterBaseIn {
    type ArrangedAccounts = SwapRouterBaseInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let input_token_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;

        Some(SwapRouterBaseInInstructionAccounts {
            payer,
            input_token_account,
            input_token_mint,
            token_program,
            token_program_2022,
            memo_program,
        })
    }
}
