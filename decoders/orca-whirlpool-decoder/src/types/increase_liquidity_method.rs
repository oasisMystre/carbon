use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum IncreaseLiquidityMethod {
    ByTokenAmounts {
        token_max_a: u64,
        token_max_b: u64,
        min_sqrt_price: u128,
        max_sqrt_price: u128,
    },
}
