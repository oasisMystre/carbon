

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}
