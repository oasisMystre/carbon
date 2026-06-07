use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePositionLockReleasePoint {
    pub position: solana_pubkey::Pubkey,
    pub current_point: u64,
    pub new_lock_release_point: u64,
    pub old_lock_release_point: u64,
    pub sender: solana_pubkey::Pubkey,
}
