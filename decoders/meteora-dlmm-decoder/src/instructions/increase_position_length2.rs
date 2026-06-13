use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xffd2cc477389e171")]
pub struct IncreasePositionLength2 {
    pub minimum_upper_bin_id: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionLength2InstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionLength2 {
    type ArrangedAccounts = IncreasePositionLength2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(IncreasePositionLength2InstructionAccounts {
            funder,
            lb_pair,
            position,
            owner,
            system_program,
            event_authority,
            program,
        })
    }
}
