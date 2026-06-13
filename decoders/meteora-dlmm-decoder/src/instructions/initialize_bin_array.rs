use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x235613b94ed44bd3")]
pub struct InitializeBinArray {
    pub index: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeBinArrayInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeBinArray {
    type ArrangedAccounts = InitializeBinArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let bin_array = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeBinArrayInstructionAccounts {
            lb_pair,
            bin_array,
            funder,
            system_program,
        })
    }
}
