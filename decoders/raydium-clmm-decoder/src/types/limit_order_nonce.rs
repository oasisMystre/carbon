use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LimitOrderNonce {
    pub user_wallet: solana_pubkey::Pubkey,
    pub nonce_index: u8,
    pub order_nonce: u64,
    pub padding: [u64; 4],
}
