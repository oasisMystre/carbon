use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1199ec3919c7e7f8")]
pub struct LimitOrderNonce {
    pub user_wallet: solana_pubkey::Pubkey,
    pub nonce_index: u8,
    pub order_nonce: u64,
    pub padding: [u64; 4],
}
