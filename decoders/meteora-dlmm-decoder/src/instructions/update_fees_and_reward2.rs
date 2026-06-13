use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x208eb89a6741b858")]
pub struct UpdateFeesAndReward2 {
    pub min_bin_id: i32,
    pub max_bin_id: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeesAndReward2InstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeesAndReward2 {
    type ArrangedAccounts = UpdateFeesAndReward2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let position = next_account(&mut iter)?;
        let lb_pair = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;

        Some(UpdateFeesAndReward2InstructionAccounts {
            position,
            lb_pair,
            owner,
        })
    }
}
