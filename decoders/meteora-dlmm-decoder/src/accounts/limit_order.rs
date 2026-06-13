use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x89b7d45b731d8de3")]
pub struct LimitOrder {
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub bin_count: u16,
    pub padding_0: [u8; 14],
    pub padding_1: [u64; 4],
}
