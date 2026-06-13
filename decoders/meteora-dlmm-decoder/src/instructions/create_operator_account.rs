use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd40f695f099e5a3")]
pub struct CreateOperatorAccount {
    pub permission: u128,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOperatorAccountInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub whitelisted_signer: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOperatorAccount {
    type ArrangedAccounts = CreateOperatorAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let operator = next_account(&mut iter)?;
        let whitelisted_signer = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateOperatorAccountInstructionAccounts {
            operator,
            whitelisted_signer,
            signer,
            payer,
            system_program,
        })
    }
}
