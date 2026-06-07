use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdffd79793cc1811f")]
pub struct IdlInclude {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IdlIncludeInstructionAccounts {
    pub tick_array: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IdlInclude {
    type ArrangedAccounts = IdlIncludeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tick_array = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(IdlIncludeInstructionAccounts {
            tick_array,
            system_program,
        })
    }
}
