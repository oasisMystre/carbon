use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbd0eb5785576e33e")]
pub struct CreateDynamicFeeConfig {
    pub index: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub dynamic_fee_control: u32,
    pub max_volatility_accumulator: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateDynamicFeeConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub dynamic_fee_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateDynamicFeeConfig {
    type ArrangedAccounts = CreateDynamicFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let dynamic_fee_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateDynamicFeeConfigInstructionAccounts {
            owner,
            dynamic_fee_config,
            system_program,
        })
    }
}
