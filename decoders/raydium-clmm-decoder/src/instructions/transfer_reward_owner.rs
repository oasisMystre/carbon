use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07160c53f22b3079")]
pub struct TransferRewardOwner {
    pub new_owner: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferRewardOwnerInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferRewardOwner {
    type ArrangedAccounts = TransferRewardOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;

        Some(TransferRewardOwnerInstructionAccounts {
            authority,
            pool_state,
        })
    }
}
