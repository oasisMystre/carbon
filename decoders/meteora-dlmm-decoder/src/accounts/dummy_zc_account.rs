use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e6bee50d030b408")]
pub struct DummyZcAccount {
    pub position_bin_data: PositionBinData,
    pub limit_order_bin_data: LimitOrderBinData,
}
