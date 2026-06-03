

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6c92566eb3fe0a68")]
pub struct CloseTokenBadge{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenBadgeInstructionAccounts {
    pub token_badge: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseTokenBadge {
    type ArrangedAccounts = CloseTokenBadgeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            token_badge,
            rent_receiver,
            admin,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseTokenBadgeInstructionAccounts {
            token_badge: token_badge.pubkey,
            rent_receiver: rent_receiver.pubkey,
            admin: admin.pubkey,
        })
    }
}