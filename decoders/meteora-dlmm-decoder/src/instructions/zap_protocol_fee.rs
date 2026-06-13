use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd59bbb2238b65bf0")]
pub struct ZapProtocolFee {
    pub max_amount: u64,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ZapProtocolFeeInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub receiver_token: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ZapProtocolFee {
    type ArrangedAccounts = ZapProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let reserve = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let receiver_token = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;

        Some(ZapProtocolFeeInstructionAccounts {
            lb_pair,
            reserve,
            token_mint,
            receiver_token,
            operator,
            signer,
            token_program,
            sysvar_instructions,
        })
    }
}
