use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe05841218a93f689")]
pub struct SetTokenBadgeAttribute {
    pub attribute: TokenBadgeAttribute,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetTokenBadgeAttributeInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpools_config_extension: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_badge: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenBadgeAttribute {
    type ArrangedAccounts = SetTokenBadgeAttributeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpools_config_extension = next_account(&mut iter)?;
        let token_badge_authority = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let token_badge = next_account(&mut iter)?;

        Some(SetTokenBadgeAttributeInstructionAccounts {
            whirlpools_config,
            whirlpools_config_extension,
            token_badge_authority,
            token_mint,
            token_badge,
        })
    }
}
