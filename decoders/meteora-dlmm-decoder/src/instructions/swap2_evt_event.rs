use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2e7452d4941b544d")]
pub struct Swap2EvtEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub from: solana_pubkey::Pubkey,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub swap_for_y: bool,
    pub fee_bps: u128,
    pub amount_in: u64,
    pub amount_left: u64,
    pub amount_out: u64,
    pub mm_fee: u64,
    pub protocol_fee: u64,
    pub limit_order_fee: u64,
    pub host_fee: u64,
    pub fees_on_input: bool,
    pub fees_on_token_x: bool,
}
