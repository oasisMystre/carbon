use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x397c249b7ef95dab")]
pub struct CloseLimitOrderIfEmpty {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseLimitOrderIfEmptyInstructionAccounts {
    pub limit_order: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseLimitOrderIfEmpty {
    type ArrangedAccounts = CloseLimitOrderIfEmptyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let limit_order = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CloseLimitOrderIfEmptyInstructionAccounts {
            limit_order,
            owner,
            rent_receiver,
            event_authority,
            program,
        })
    }
}
