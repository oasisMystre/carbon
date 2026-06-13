use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb1fbc91458bcc75")]
pub struct Operator {
    pub signer: solana_pubkey::Pubkey,
    pub permission: u128,
    pub padding: [u64; 2],
}
