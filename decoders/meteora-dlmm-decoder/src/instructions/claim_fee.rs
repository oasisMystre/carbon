use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9204f8988e84689")]
pub struct ClaimFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimFeeInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFee {
    type ArrangedAccounts = ClaimFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let bin_array_lower = next_account(&mut iter)?;
        let bin_array_upper = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let user_token_x = next_account(&mut iter)?;
        let user_token_y = next_account(&mut iter)?;
        let token_x_mint = next_account(&mut iter)?;
        let token_y_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(ClaimFeeInstructionAccounts {
            lb_pair,
            position,
            bin_array_lower,
            bin_array_upper,
            sender,
            reserve_x,
            reserve_y,
            user_token_x,
            user_token_y,
            token_x_mint,
            token_y_mint,
            token_program,
            event_authority,
            program,
        })
    }
}
