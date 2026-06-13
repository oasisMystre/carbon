use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e527d92558de499")]
pub struct InitializePositionPda {
    pub lower_bin_id: i32,
    pub width: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePositionPdaInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub base: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionPda {
    type ArrangedAccounts = InitializePositionPdaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let base = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializePositionPdaInstructionAccounts {
            payer,
            base,
            position,
            lb_pair,
            owner,
            system_program,
            rent,
            event_authority,
            program,
        })
    }
}
