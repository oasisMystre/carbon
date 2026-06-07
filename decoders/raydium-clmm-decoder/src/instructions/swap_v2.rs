use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b04ed0b1ac91e62")]
pub struct SwapV2 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapV2InstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub output_vault: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub input_vault_mint: solana_pubkey::Pubkey,
    pub output_vault_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapV2 {
    type ArrangedAccounts = SwapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let input_vault = next_account(&mut iter)?;
        let output_vault = next_account(&mut iter)?;
        let observation_state = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let input_vault_mint = next_account(&mut iter)?;
        let output_vault_mint = next_account(&mut iter)?;

        Some(SwapV2InstructionAccounts {
            payer,
            amm_config,
            pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            observation_state,
            token_program,
            token_program_2022,
            memo_program,
            input_vault_mint,
            output_vault_mint,
        })
    }
}
