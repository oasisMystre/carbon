use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6a1f84f9862ace7")]
pub struct MigrateRepurposeRewardAuthoritySpace {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrateRepurposeRewardAuthoritySpaceInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateRepurposeRewardAuthoritySpace {
    type ArrangedAccounts = MigrateRepurposeRewardAuthoritySpaceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;

        Some(MigrateRepurposeRewardAuthoritySpaceInstructionAccounts { whirlpool })
    }
}
