use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionBinData {
    pub liquidity_share: u128,
    pub reward_info: UserRewardInfo,
    pub fee_info: FeeInfo,
}
