use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa3ace0340b9a6adf")]
pub struct UpdateRewardInfos {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardInfosInstructionAccounts {
    pub pool_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardInfos {
    type ArrangedAccounts = UpdateRewardInfosInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_state = next_account(&mut iter)?;

        Some(UpdateRewardInfosInstructionAccounts { pool_state })
    }
}
