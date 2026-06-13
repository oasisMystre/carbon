use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Operator {
    pub signer: solana_pubkey::Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}
