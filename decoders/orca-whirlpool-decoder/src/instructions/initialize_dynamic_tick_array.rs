use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2921a5c878e78e32")]
pub struct InitializeDynamicTickArray {
    pub start_tick_index: i32,
    pub idempotent: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeDynamicTickArrayInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub tick_array: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeDynamicTickArray {
    type ArrangedAccounts = InitializeDynamicTickArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let tick_array = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeDynamicTickArrayInstructionAccounts {
            whirlpool,
            funder,
            tick_array,
            system_program,
        })
    }
}
