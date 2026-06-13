use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb8d5581fb3658224")]
pub struct CloseClaimFeeOperatorAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseClaimFeeOperatorAccountInstructionAccounts {
    pub claim_fee_operator: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseClaimFeeOperatorAccount {
    type ArrangedAccounts = CloseClaimFeeOperatorAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let claim_fee_operator = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(CloseClaimFeeOperatorAccountInstructionAccounts {
            claim_fee_operator,
            rent_receiver,
            signer,
        })
    }
}
