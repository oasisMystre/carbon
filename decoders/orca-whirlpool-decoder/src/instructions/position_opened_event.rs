use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xedaff3e693756579")]
pub struct PositionOpenedEvent {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}
