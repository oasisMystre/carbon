use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DummyIx {
    pub pair_status: PairStatus,
    pub pair_type: PairType,
    pub activation_type: ActivationType,
    pub token_program_flag: TokenProgramFlags,
    pub resize_side: ResizeSide,
    pub rounding: Rounding,
}
