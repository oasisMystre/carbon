use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct IncreaseObservation {
    pub oracle: solana_pubkey::Pubkey,
    pub new_observation_length: u64,
}
