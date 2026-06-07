use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c04b0c177b95309")]
pub struct RebalanceLiquidity {
    pub params: RebalanceLiquidityParams,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RebalanceLiquidityInstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RebalanceLiquidity {
    type ArrangedAccounts = RebalanceLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_bitmap_extension, user_token_x, user_token_y, reserve_x, reserve_y, token_x_mint, token_y_mint, owner, rent_payer, token_x_program, token_y_program, memo_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RebalanceLiquidityInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token_x: user_token_x.pubkey,
            user_token_y: user_token_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            owner: owner.pubkey,
            rent_payer: rent_payer.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            memo_program: memo_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
