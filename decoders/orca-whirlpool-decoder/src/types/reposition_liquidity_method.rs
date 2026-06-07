use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum RepositionLiquidityMethod {
    ByLiquidity {
        new_liquidity_amount: u128,
        existing_range_token_min_a: u64,
        existing_range_token_min_b: u64,
        new_range_token_max_a: u64,
        new_range_token_max_b: u64,
    },
}
