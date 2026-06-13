use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x44ae5850b5cc13e0")]
pub struct CloseBinArray {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseBinArrayInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseBinArray {
    type ArrangedAccounts = CloseBinArrayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let bin_array = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(CloseBinArrayInstructionAccounts {
            lb_pair,
            bin_array,
            rent_receiver,
            signer,
        })
    }
}
