use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x505375d3420d2195")]
pub struct IncreasePositionLength {
    pub length_to_add: u16,
    pub side: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IncreasePositionLengthInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionLength {
    type ArrangedAccounts = IncreasePositionLengthInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funder, lb_pair, position, owner, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreasePositionLengthInstructionAccounts {
            funder: funder.pubkey,
            lb_pair: lb_pair.pubkey,
            position: position.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
