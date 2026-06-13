use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x43f8e7899a95d9ae")]
pub struct SetPairStatus {
    pub status: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPairStatus {
    type ArrangedAccounts = SetPairStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(SetPairStatusInstructionAccounts {
            lb_pair,
            operator,
            signer,
        })
    }
}
