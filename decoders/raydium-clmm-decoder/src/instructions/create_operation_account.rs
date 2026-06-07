use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3f5794216d230868")]
pub struct CreateOperationAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOperationAccountInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub operation_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOperationAccount {
    type ArrangedAccounts = CreateOperationAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let operation_state = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateOperationAccountInstructionAccounts {
            owner,
            operation_state,
            system_program,
        })
    }
}
