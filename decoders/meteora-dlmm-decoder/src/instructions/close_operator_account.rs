use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xab09d54a7817031d")]
pub struct CloseOperatorAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseOperatorAccountInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOperatorAccount {
    type ArrangedAccounts = CloseOperatorAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;

        Some(CloseOperatorAccountInstructionAccounts {
            operator,
            signer,
            rent_receiver,
        })
    }
}
