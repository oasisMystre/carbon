use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce57114f2d29d53d")]
pub struct CollectProtocolFeeEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub recipient_token_account_0: solana_pubkey::Pubkey,
    pub recipient_token_account_1: solana_pubkey::Pubkey,
    pub amount_0: u64,
    pub amount_1: u64,
}
