use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x859ed4bded0c4927")]
pub struct SetAdaptiveFeeConstants {
    pub filter_period: Option<u16>,
    pub decay_period: Option<u16>,
    pub reduction_factor: Option<u16>,
    pub adaptive_fee_control_factor: Option<u32>,
    pub max_volatility_accumulator: Option<u32>,
    pub tick_group_size: Option<u16>,
    pub major_swap_threshold_ticks: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetAdaptiveFeeConstantsInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAdaptiveFeeConstants {
    type ArrangedAccounts = SetAdaptiveFeeConstantsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let whirlpools_config = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;

        Some(SetAdaptiveFeeConstantsInstructionAccounts {
            whirlpool,
            whirlpools_config,
            oracle,
            fee_authority,
        })
    }
}
