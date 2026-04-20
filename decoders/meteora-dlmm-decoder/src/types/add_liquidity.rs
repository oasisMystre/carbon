

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddLiquidity {
    pub lb_pair: solana_pubkey::Pubkey,
    pub from: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub amounts: [u64; 2],
    pub active_bin_id: i32,
}
