use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfbbdbef475fe2394")]
pub struct InitializePositionByOperator {
    pub lower_bin_id: i32,
    pub width: i32,
    pub fee_owner: solana_pubkey::Pubkey,
    pub lock_release_point: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionByOperatorInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub operator_token_x: solana_pubkey::Pubkey,
    pub owner_token_x: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionByOperator {
    type ArrangedAccounts = InitializePositionByOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let base = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let operator_token_x = next_account(&mut iter)?;
        let owner_token_x = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializePositionByOperatorInstructionAccounts {
            payer,
            base,
            position,
            lb_pair,
            owner,
            operator,
            operator_token_x,
            owner_token_x,
            system_program,
            event_authority,
            program,
        })
    }
}
