use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d8e87084c5c3f7653")]
pub struct CloseLimitOrderEvtEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub limit_order: solana_pubkey::Pubkey,
}
