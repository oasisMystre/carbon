

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CompositionFee {
    pub from: solana_pubkey::Pubkey,
    pub bin_id: i16,
    pub token_x_fee_amount: u64,
    pub token_y_fee_amount: u64,
    pub protocol_token_x_fee_amount: u64,
    pub protocol_token_y_fee_amount: u64,
}
