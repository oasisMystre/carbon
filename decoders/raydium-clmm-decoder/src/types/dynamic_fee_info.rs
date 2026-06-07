use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DynamicFeeInfo {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub dynamic_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub tick_spacing_index_reference: i32,
    pub volatility_reference: u32,
    pub volatility_accumulator: u32,
    pub last_update_timestamp: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 46],
}
