use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc975989055556cb2")]
pub struct CloseProtocolPosition {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseProtocolPositionInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseProtocolPosition {
    type ArrangedAccounts = CloseProtocolPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let protocol_position = next_account(&mut iter)?;

        Some(CloseProtocolPositionInstructionAccounts {
            admin,
            protocol_position,
        })
    }
}
