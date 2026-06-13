use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BinArray {
    pub index: i64,
    pub version: u8,
    pub padding_1: [u8; 7],
    pub lb_pair: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub bins: [Bin; 70],
}
