use solana_pubkey::Pubkey;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d63f91179a69ccfd7")]
pub struct IncreaseObservationEvent {
    pub oracle: Pubkey,
    pub new_observation_length: u64,
}
