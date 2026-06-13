use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2b4f1ba9f41ce13f")]
pub struct PlaceLimitOrderEvtEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
    pub active_id: i32,
    pub params: PlaceLimitOrderParams,
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlaceLimitOrderParams {
    pub is_ask_side: bool,
    pub padding: [u8; 16],
    pub relative_bin: Option<RelativeBin>,
    pub bins: Vec<BinLimitOrderAmount>,
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RelativeBin {
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BinLimitOrderAmount {
    pub id: i32,
    pub amount: u64,
}
