use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03dd95da6f8d76d5")]
pub struct AddLiquidityByStrategy2 {
    pub liquidity_parameter: LiquidityParameterByStrategy,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityByStrategy2InstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityByStrategy2 {
    type ArrangedAccounts = AddLiquidityByStrategy2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let user_token_x = next_account(&mut iter)?;
        let user_token_y = next_account(&mut iter)?;
        let reserve_x = next_account(&mut iter)?;
        let reserve_y = next_account(&mut iter)?;
        let token_x_mint = next_account(&mut iter)?;
        let token_y_mint = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let token_x_program = next_account(&mut iter)?;
        let token_y_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AddLiquidityByStrategy2InstructionAccounts {
            position,
            lb_pair,
            bin_array_bitmap_extension,
            user_token_x,
            user_token_y,
            reserve_x,
            reserve_y,
            token_x_mint,
            token_y_mint,
            sender,
            token_x_program,
            token_y_program,
            event_authority,
            program,
        })
    }
}
