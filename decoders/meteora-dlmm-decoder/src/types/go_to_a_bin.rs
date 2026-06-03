

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct GoToABin {
    pub lb_pair: solana_pubkey::Pubkey,
    pub from_bin_id: i32,
    pub to_bin_id: i32,
}
