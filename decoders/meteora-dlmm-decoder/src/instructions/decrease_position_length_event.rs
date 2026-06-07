use solana_pubkey::Pubkey;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3476eb55aca90f80")]
pub struct DecreasePositionLengthEvent {
    pub lb_pair: Pubkey,
    pub position: Pubkey,
    pub owner: Pubkey,
    pub length_to_remove: u16,
    pub side: u8,
}
