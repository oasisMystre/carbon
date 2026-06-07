use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc2db882019606925")]
pub struct DecreasePositionLength {
    pub length_to_remove: u16,
    pub side: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecreasePositionLengthInstructionAccounts {
    pub rent_receiver: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreasePositionLength {
    type ArrangedAccounts = DecreasePositionLengthInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [rent_receiver, position, owner, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DecreasePositionLengthInstructionAccounts {
            rent_receiver: rent_receiver.pubkey,
            position: position.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
