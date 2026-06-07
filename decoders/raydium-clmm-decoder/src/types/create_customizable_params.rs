use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateCustomizableParams {
    pub sqrt_price_x64: u128,
    pub collect_fee_on: CollectFeeOn,
    pub enable_dynamic_fee: bool,
}
