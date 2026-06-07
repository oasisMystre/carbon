use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x5e6bee50d030b408")]
pub struct DummyZcAccount {
    pub position_bin_data: PositionBinData,
}
