use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa53dc9f4829f1664")]
pub struct SetPreActivationDuration {
    pub pre_activation_duration: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPreActivationDurationInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPreActivationDuration {
    type ArrangedAccounts = SetPreActivationDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(SetPreActivationDurationInstructionAccounts { lb_pair, signer })
    }
}
