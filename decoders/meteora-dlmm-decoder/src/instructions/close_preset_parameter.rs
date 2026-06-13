use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04949164861ab53d")]
pub struct ClosePresetParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePresetParameterInstructionAccounts {
    pub preset_parameter: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePresetParameter {
    type ArrangedAccounts = ClosePresetParameterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let preset_parameter = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;

        Some(ClosePresetParameterInstructionAccounts {
            preset_parameter,
            operator,
            signer,
            rent_receiver,
        })
    }
}
