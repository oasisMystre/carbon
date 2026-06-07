use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x47ade41243f7d239")]
pub struct SetConfigFeatureFlag {
    pub feature_flag: ConfigFeatureFlag,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetConfigFeatureFlagInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetConfigFeatureFlag {
    type ArrangedAccounts = SetConfigFeatureFlagInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;

        Some(SetConfigFeatureFlagInstructionAccounts {
            whirlpools_config,
            authority,
        })
    }
}
