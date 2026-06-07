use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11fb415c88f20ea9")]
pub struct CreateSupportMintAssociated {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateSupportMintAssociatedInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub support_mint_associated: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateSupportMintAssociated {
    type ArrangedAccounts = CreateSupportMintAssociatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let support_mint_associated = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateSupportMintAssociatedInstructionAccounts {
            owner,
            token_mint,
            support_mint_associated,
            system_program,
        })
    }
}
