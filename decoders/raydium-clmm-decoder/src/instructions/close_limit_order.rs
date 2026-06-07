use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4c7c800fd55725fa")]
pub struct CloseLimitOrder {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseLimitOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseLimitOrder {
    type ArrangedAccounts = CloseLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let limit_order = next_account(&mut iter)?;

        Some(CloseLimitOrderInstructionAccounts {
            signer,
            rent_receiver,
            limit_order,
        })
    }
}
