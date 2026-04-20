

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct FeeParameterUpdate {
    pub lb_pair: solana_pubkey::Pubkey,
    pub protocol_share: u16,
    pub base_factor: u16,
}
