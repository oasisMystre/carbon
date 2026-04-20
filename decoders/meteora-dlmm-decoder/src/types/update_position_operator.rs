

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdatePositionOperator {
    pub position: solana_pubkey::Pubkey,
    pub old_operator: solana_pubkey::Pubkey,
    pub new_operator: solana_pubkey::Pubkey,
}
