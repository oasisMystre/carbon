use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x398b2f7bd850df0a")]
pub struct SetPreActivationSwapAddress {
    pub pre_activation_swap_address: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPreActivationSwapAddressInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPreActivationSwapAddress {
    type ArrangedAccounts = SetPreActivationSwapAddressInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(SetPreActivationSwapAddressInstructionAccounts { lb_pair, signer })
    }
}
