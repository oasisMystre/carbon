use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e3b98d346b72ed0")]
pub struct SetPairStatusPermissionless {
    pub status: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPairStatusPermissionlessInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPairStatusPermissionless {
    type ArrangedAccounts = SetPairStatusPermissionlessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lb_pair = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        Some(SetPairStatusPermissionlessInstructionAccounts { lb_pair, signer })
    }
}
