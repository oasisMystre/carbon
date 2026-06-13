use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LimitOrderBinData {
    pub amount: u64,
    pub age: u32,
    pub padding_0: [u8; 4],
    pub bin_id: i32,
    pub is_ask: u8,
    pub padding_1: [u8; 11],
}
