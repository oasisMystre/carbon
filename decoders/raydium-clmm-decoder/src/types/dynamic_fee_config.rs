use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DynamicFeeConfig {
    pub index: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub dynamic_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub padding: [u64; 8],
}
