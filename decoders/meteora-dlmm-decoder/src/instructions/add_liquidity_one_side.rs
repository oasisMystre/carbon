use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e9b6797465fdca5")]
pub struct AddLiquidityOneSide {
    pub liquidity_parameter: LiquidityOneSideParameter,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityOneSideInstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub bin_array_lower: solana_pubkey::Pubkey,
    pub bin_array_upper: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidityOneSide {
    type ArrangedAccounts = AddLiquidityOneSideInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let bin_array_bitmap_extension = next_account(&mut iter)?;
        let user_token = next_account(&mut iter)?;
        let reserve = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let bin_array_lower = next_account(&mut iter)?;
        let bin_array_upper = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(AddLiquidityOneSideInstructionAccounts {
            position,
            lb_pair,
            bin_array_bitmap_extension,
            user_token,
            reserve,
            token_mint,
            bin_array_lower,
            bin_array_upper,
            sender,
            token_program,
            event_authority,
            program,
        })
    }
}
