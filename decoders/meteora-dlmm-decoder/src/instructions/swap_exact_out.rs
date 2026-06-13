use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfa49652126cf4bb8")]
pub struct SwapExactOut {
    pub max_in_amount: u64,
    pub out_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapExactOutInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub user_token_in: solana_pubkey::Pubkey,
    pub user_token_out: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub host_fee_in: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapExactOut {
    type ArrangedAccounts = SwapExactOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let user_token_in = next_account(&mut iter)?;
        let user_token_out = next_account(&mut iter)?;
        let token_x_mint = next_account(&mut iter)?;
        let token_y_mint = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let host_fee_in = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let token_x_program = next_account(&mut iter)?;
        let token_y_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SwapExactOutInstructionAccounts {
            lb_pair,
            bin_array_bitmap_extension,
            reserve_x,
            reserve_y,
            user_token_in,
            user_token_out,
            token_x_mint,
            token_y_mint,
            oracle,
            host_fee_in,
            user,
            token_x_program,
            token_y_program,
            event_authority,
            program,
        })
    }
}
