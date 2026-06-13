use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlaceLimitOrderEvt {
    pub lb_pair: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub active_id: i32,
    pub params: PlaceLimitOrderParams,
}
