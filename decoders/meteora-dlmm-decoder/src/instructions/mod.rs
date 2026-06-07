use super::MeteoraDlmmDecoder;
pub mod add_liquidity;
pub mod add_liquidity2;
pub mod add_liquidity_by_strategy;
pub mod add_liquidity_by_strategy2;
pub mod add_liquidity_by_strategy_one_side;
pub mod add_liquidity_by_weight;
pub mod add_liquidity_event;
pub mod add_liquidity_one_side;
pub mod add_liquidity_one_side_precise;
pub mod add_liquidity_one_side_precise2;
pub mod claim_fee;
pub mod claim_fee2;
pub mod claim_fee2_event;
pub mod claim_fee_event;
pub mod claim_reward;
pub mod claim_reward2;
pub mod claim_reward2_event;
pub mod claim_reward_event;
pub mod close_claim_protocol_fee_operator;
pub mod close_position;
pub mod close_position2;
pub mod close_position_if_empty;
pub mod close_preset_parameter;
pub mod close_preset_parameter2;
pub mod close_token_badge;
pub mod composition_fee_event;
pub mod create_claim_protocol_fee_operator;
pub mod decrease_position_length;
pub mod decrease_position_length_event;
pub mod dynamic_fee_parameter_update_event;
pub mod fee_parameter_update_event;
pub mod for_idl_type_generation_do_not_call;
pub mod fund_reward;
pub mod fund_reward_event;
pub mod go_to_a_bin;
pub mod go_to_a_bin_event;
pub mod increase_observation_event;
pub mod increase_oracle_length;
pub mod increase_position_length;
pub mod increase_position_length2;
pub mod increase_position_length_event;
pub mod initialize_bin_array;
pub mod initialize_bin_array_bitmap_extension;
pub mod initialize_customizable_permissionless_lb_pair;
pub mod initialize_customizable_permissionless_lb_pair2;
pub mod initialize_lb_pair;
pub mod initialize_lb_pair2;
pub mod initialize_permission_lb_pair;
pub mod initialize_position;
pub mod initialize_position2;
pub mod initialize_position_by_operator;
pub mod initialize_position_pda;
pub mod initialize_preset_parameter;
pub mod initialize_preset_parameter2;
pub mod initialize_reward;
pub mod initialize_reward_event;
pub mod initialize_token_badge;
pub mod lb_pair_create_event;
pub mod migrate_bin_array;
pub mod migrate_position;
pub mod position_close_event;
pub mod position_create_event;
pub mod rebalance_liquidity;
pub mod rebalancing_event;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod remove_liquidity2;
pub mod remove_liquidity_by_range;
pub mod remove_liquidity_by_range2;
pub mod remove_liquidity_event;
pub mod set_activation_point;
pub mod set_pair_status;
pub mod set_pair_status_permissionless;
pub mod set_pre_activation_duration;
pub mod set_pre_activation_swap_address;
pub mod swap;
pub mod swap2;
pub mod swap_event;
pub mod swap_exact_out;
pub mod swap_exact_out2;
pub mod swap_with_price_impact;
pub mod swap_with_price_impact2;
pub mod update_base_fee_parameters;
pub mod update_dynamic_fee_parameters;
pub mod update_fees_and_reward2;
pub mod update_fees_and_rewards;
pub mod update_position_lock_release_point_event;
pub mod update_position_operator;
pub mod update_position_operator_event;
pub mod update_reward_duration;
pub mod update_reward_duration_event;
pub mod update_reward_funder;
pub mod update_reward_funder_event;
pub mod withdraw_ineligible_reward;
pub mod withdraw_ineligible_reward_event;
pub mod withdraw_protocol_fee;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MeteoraDlmmInstruction {
    AddLiquidity(add_liquidity::AddLiquidity),
    AddLiquidity2(add_liquidity2::AddLiquidity2),
    AddLiquidityByStrategy(add_liquidity_by_strategy::AddLiquidityByStrategy),
    AddLiquidityByStrategy2(add_liquidity_by_strategy2::AddLiquidityByStrategy2),
    AddLiquidityByStrategyOneSide(add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide),
    AddLiquidityByWeight(add_liquidity_by_weight::AddLiquidityByWeight),
    AddLiquidityOneSide(add_liquidity_one_side::AddLiquidityOneSide),
    AddLiquidityOneSidePrecise(add_liquidity_one_side_precise::AddLiquidityOneSidePrecise),
    AddLiquidityOneSidePrecise2(add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2),
    ClaimFee(claim_fee::ClaimFee),
    ClaimFee2(claim_fee2::ClaimFee2),
    ClaimReward(claim_reward::ClaimReward),
    ClaimReward2(claim_reward2::ClaimReward2),
    CloseClaimProtocolFeeOperator(close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator),
    ClosePosition(close_position::ClosePosition),
    ClosePosition2(close_position2::ClosePosition2),
    ClosePositionIfEmpty(close_position_if_empty::ClosePositionIfEmpty),
    ClosePresetParameter(close_preset_parameter::ClosePresetParameter),
    ClosePresetParameter2(close_preset_parameter2::ClosePresetParameter2),
    CloseTokenBadge(close_token_badge::CloseTokenBadge),
    CreateClaimProtocolFeeOperator(create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator),
    DecreasePositionLength(decrease_position_length::DecreasePositionLength),
    ForIdlTypeGenerationDoNotCall(for_idl_type_generation_do_not_call::ForIdlTypeGenerationDoNotCall),
    FundReward(fund_reward::FundReward),
    GoToABin(go_to_a_bin::GoToABin),
    IncreaseOracleLength(increase_oracle_length::IncreaseOracleLength),
    IncreasePositionLength(increase_position_length::IncreasePositionLength),
    IncreasePositionLength2(increase_position_length2::IncreasePositionLength2),
    InitializeBinArray(initialize_bin_array::InitializeBinArray),
    InitializeBinArrayBitmapExtension(initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension),
    InitializeCustomizablePermissionlessLbPair(initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair),
    InitializeCustomizablePermissionlessLbPair2(initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2),
    InitializeLbPair(initialize_lb_pair::InitializeLbPair),
    InitializeLbPair2(initialize_lb_pair2::InitializeLbPair2),
    InitializePermissionLbPair(initialize_permission_lb_pair::InitializePermissionLbPair),
    InitializePosition(initialize_position::InitializePosition),
    InitializePosition2(initialize_position2::InitializePosition2),
    InitializePositionByOperator(initialize_position_by_operator::InitializePositionByOperator),
    InitializePositionPda(initialize_position_pda::InitializePositionPda),
    InitializePresetParameter(initialize_preset_parameter::InitializePresetParameter),
    InitializePresetParameter2(initialize_preset_parameter2::InitializePresetParameter2),
    InitializeReward(initialize_reward::InitializeReward),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    MigrateBinArray(migrate_bin_array::MigrateBinArray),
    MigratePosition(migrate_position::MigratePosition),
    RebalanceLiquidity(rebalance_liquidity::RebalanceLiquidity),
    RemoveAllLiquidity(remove_all_liquidity::RemoveAllLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    RemoveLiquidity2(remove_liquidity2::RemoveLiquidity2),
    RemoveLiquidityByRange(remove_liquidity_by_range::RemoveLiquidityByRange),
    RemoveLiquidityByRange2(remove_liquidity_by_range2::RemoveLiquidityByRange2),
    SetActivationPoint(set_activation_point::SetActivationPoint),
    SetPairStatus(set_pair_status::SetPairStatus),
    SetPairStatusPermissionless(set_pair_status_permissionless::SetPairStatusPermissionless),
    SetPreActivationDuration(set_pre_activation_duration::SetPreActivationDuration),
    SetPreActivationSwapAddress(set_pre_activation_swap_address::SetPreActivationSwapAddress),
    Swap(swap::Swap),
    Swap2(swap2::Swap2),
    SwapExactOut(swap_exact_out::SwapExactOut),
    SwapExactOut2(swap_exact_out2::SwapExactOut2),
    SwapWithPriceImpact(swap_with_price_impact::SwapWithPriceImpact),
    SwapWithPriceImpact2(swap_with_price_impact2::SwapWithPriceImpact2),
    UpdateBaseFeeParameters(update_base_fee_parameters::UpdateBaseFeeParameters),
    UpdateDynamicFeeParameters(update_dynamic_fee_parameters::UpdateDynamicFeeParameters),
    UpdateFeesAndReward2(update_fees_and_reward2::UpdateFeesAndReward2),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    UpdatePositionOperator(update_position_operator::UpdatePositionOperator),
    UpdateRewardDuration(update_reward_duration::UpdateRewardDuration),
    UpdateRewardFunder(update_reward_funder::UpdateRewardFunder),
    WithdrawIneligibleReward(withdraw_ineligible_reward::WithdrawIneligibleReward),
    WithdrawProtocolFee(withdraw_protocol_fee::WithdrawProtocolFee),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    ClaimFeeEvent(claim_fee_event::ClaimFeeEvent),
    ClaimFee2Event(claim_fee2_event::ClaimFee2Event),
    ClaimRewardEvent(claim_reward_event::ClaimRewardEvent),
    ClaimReward2Event(claim_reward2_event::ClaimReward2Event),
    CompositionFeeEvent(composition_fee_event::CompositionFeeEvent),
    DecreasePositionLengthEvent(decrease_position_length_event::DecreasePositionLengthEvent),
    DynamicFeeParameterUpdateEvent(dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent),
    FeeParameterUpdateEvent(fee_parameter_update_event::FeeParameterUpdateEvent),
    FundRewardEvent(fund_reward_event::FundRewardEvent),
    GoToABinEvent(go_to_a_bin_event::GoToABinEvent),
    IncreaseObservationEvent(increase_observation_event::IncreaseObservationEvent),
    IncreasePositionLengthEvent(increase_position_length_event::IncreasePositionLengthEvent),
    InitializeRewardEvent(initialize_reward_event::InitializeRewardEvent),
    LbPairCreateEvent(lb_pair_create_event::LbPairCreateEvent),
    PositionCloseEvent(position_close_event::PositionCloseEvent),
    PositionCreateEvent(position_create_event::PositionCreateEvent),
    RebalancingEvent(rebalancing_event::RebalancingEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    SwapEvent(swap_event::SwapEvent),
    UpdatePositionLockReleasePointEvent(update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent),
    UpdatePositionOperatorEvent(update_position_operator_event::UpdatePositionOperatorEvent),
    UpdateRewardDurationEvent(update_reward_duration_event::UpdateRewardDurationEvent),
    UpdateRewardFunderEvent(update_reward_funder_event::UpdateRewardFunderEvent),
    WithdrawIneligibleRewardEvent(withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MeteoraDlmmDecoder {
    type InstructionType = MeteoraDlmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MeteoraDlmmInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            MeteoraDlmmInstruction::AddLiquidity2 => add_liquidity2::AddLiquidity2,
            MeteoraDlmmInstruction::AddLiquidityByStrategy => add_liquidity_by_strategy::AddLiquidityByStrategy,
            MeteoraDlmmInstruction::AddLiquidityByStrategy2 => add_liquidity_by_strategy2::AddLiquidityByStrategy2,
            MeteoraDlmmInstruction::AddLiquidityByStrategyOneSide => add_liquidity_by_strategy_one_side::AddLiquidityByStrategyOneSide,
            MeteoraDlmmInstruction::AddLiquidityByWeight => add_liquidity_by_weight::AddLiquidityByWeight,
            MeteoraDlmmInstruction::AddLiquidityOneSide => add_liquidity_one_side::AddLiquidityOneSide,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise => add_liquidity_one_side_precise::AddLiquidityOneSidePrecise,
            MeteoraDlmmInstruction::AddLiquidityOneSidePrecise2 => add_liquidity_one_side_precise2::AddLiquidityOneSidePrecise2,
            MeteoraDlmmInstruction::ClaimFee => claim_fee::ClaimFee,
            MeteoraDlmmInstruction::ClaimFee2 => claim_fee2::ClaimFee2,
            MeteoraDlmmInstruction::ClaimReward => claim_reward::ClaimReward,
            MeteoraDlmmInstruction::ClaimReward2 => claim_reward2::ClaimReward2,
            MeteoraDlmmInstruction::CloseClaimProtocolFeeOperator => close_claim_protocol_fee_operator::CloseClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::ClosePosition => close_position::ClosePosition,
            MeteoraDlmmInstruction::ClosePosition2 => close_position2::ClosePosition2,
            MeteoraDlmmInstruction::ClosePositionIfEmpty => close_position_if_empty::ClosePositionIfEmpty,
            MeteoraDlmmInstruction::ClosePresetParameter => close_preset_parameter::ClosePresetParameter,
            MeteoraDlmmInstruction::ClosePresetParameter2 => close_preset_parameter2::ClosePresetParameter2,
            MeteoraDlmmInstruction::CloseTokenBadge => close_token_badge::CloseTokenBadge,
            MeteoraDlmmInstruction::CreateClaimProtocolFeeOperator => create_claim_protocol_fee_operator::CreateClaimProtocolFeeOperator,
            MeteoraDlmmInstruction::DecreasePositionLength => decrease_position_length::DecreasePositionLength,
            MeteoraDlmmInstruction::ForIdlTypeGenerationDoNotCall => for_idl_type_generation_do_not_call::ForIdlTypeGenerationDoNotCall,
            MeteoraDlmmInstruction::FundReward => fund_reward::FundReward,
            MeteoraDlmmInstruction::GoToABin => go_to_a_bin::GoToABin,
            MeteoraDlmmInstruction::IncreaseOracleLength => increase_oracle_length::IncreaseOracleLength,
            MeteoraDlmmInstruction::IncreasePositionLength => increase_position_length::IncreasePositionLength,
            MeteoraDlmmInstruction::IncreasePositionLength2 => increase_position_length2::IncreasePositionLength2,
            MeteoraDlmmInstruction::InitializeBinArray => initialize_bin_array::InitializeBinArray,
            MeteoraDlmmInstruction::InitializeBinArrayBitmapExtension => initialize_bin_array_bitmap_extension::InitializeBinArrayBitmapExtension,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair => initialize_customizable_permissionless_lb_pair::InitializeCustomizablePermissionlessLbPair,
            MeteoraDlmmInstruction::InitializeCustomizablePermissionlessLbPair2 => initialize_customizable_permissionless_lb_pair2::InitializeCustomizablePermissionlessLbPair2,
            MeteoraDlmmInstruction::InitializeLbPair => initialize_lb_pair::InitializeLbPair,
            MeteoraDlmmInstruction::InitializeLbPair2 => initialize_lb_pair2::InitializeLbPair2,
            MeteoraDlmmInstruction::InitializePermissionLbPair => initialize_permission_lb_pair::InitializePermissionLbPair,
            MeteoraDlmmInstruction::InitializePosition => initialize_position::InitializePosition,
            MeteoraDlmmInstruction::InitializePosition2 => initialize_position2::InitializePosition2,
            MeteoraDlmmInstruction::InitializePositionByOperator => initialize_position_by_operator::InitializePositionByOperator,
            MeteoraDlmmInstruction::InitializePositionPda => initialize_position_pda::InitializePositionPda,
            MeteoraDlmmInstruction::InitializePresetParameter => initialize_preset_parameter::InitializePresetParameter,
            MeteoraDlmmInstruction::InitializePresetParameter2 => initialize_preset_parameter2::InitializePresetParameter2,
            MeteoraDlmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            MeteoraDlmmInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            MeteoraDlmmInstruction::MigrateBinArray => migrate_bin_array::MigrateBinArray,
            MeteoraDlmmInstruction::MigratePosition => migrate_position::MigratePosition,
            MeteoraDlmmInstruction::RebalanceLiquidity => rebalance_liquidity::RebalanceLiquidity,
            MeteoraDlmmInstruction::RemoveAllLiquidity => remove_all_liquidity::RemoveAllLiquidity,
            MeteoraDlmmInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            MeteoraDlmmInstruction::RemoveLiquidity2 => remove_liquidity2::RemoveLiquidity2,
            MeteoraDlmmInstruction::RemoveLiquidityByRange => remove_liquidity_by_range::RemoveLiquidityByRange,
            MeteoraDlmmInstruction::RemoveLiquidityByRange2 => remove_liquidity_by_range2::RemoveLiquidityByRange2,
            MeteoraDlmmInstruction::SetActivationPoint => set_activation_point::SetActivationPoint,
            MeteoraDlmmInstruction::SetPairStatus => set_pair_status::SetPairStatus,
            MeteoraDlmmInstruction::SetPairStatusPermissionless => set_pair_status_permissionless::SetPairStatusPermissionless,
            MeteoraDlmmInstruction::SetPreActivationDuration => set_pre_activation_duration::SetPreActivationDuration,
            MeteoraDlmmInstruction::SetPreActivationSwapAddress => set_pre_activation_swap_address::SetPreActivationSwapAddress,
            MeteoraDlmmInstruction::Swap => swap::Swap,
            MeteoraDlmmInstruction::Swap2 => swap2::Swap2,
            MeteoraDlmmInstruction::SwapExactOut => swap_exact_out::SwapExactOut,
            MeteoraDlmmInstruction::SwapExactOut2 => swap_exact_out2::SwapExactOut2,
            MeteoraDlmmInstruction::SwapWithPriceImpact => swap_with_price_impact::SwapWithPriceImpact,
            MeteoraDlmmInstruction::SwapWithPriceImpact2 => swap_with_price_impact2::SwapWithPriceImpact2,
            MeteoraDlmmInstruction::UpdateBaseFeeParameters => update_base_fee_parameters::UpdateBaseFeeParameters,
            MeteoraDlmmInstruction::UpdateDynamicFeeParameters => update_dynamic_fee_parameters::UpdateDynamicFeeParameters,
            MeteoraDlmmInstruction::UpdateFeesAndReward2 => update_fees_and_reward2::UpdateFeesAndReward2,
            MeteoraDlmmInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            MeteoraDlmmInstruction::UpdatePositionOperator => update_position_operator::UpdatePositionOperator,
            MeteoraDlmmInstruction::UpdateRewardDuration => update_reward_duration::UpdateRewardDuration,
            MeteoraDlmmInstruction::UpdateRewardFunder => update_reward_funder::UpdateRewardFunder,
            MeteoraDlmmInstruction::WithdrawIneligibleReward => withdraw_ineligible_reward::WithdrawIneligibleReward,
            MeteoraDlmmInstruction::WithdrawProtocolFee => withdraw_protocol_fee::WithdrawProtocolFee,
            MeteoraDlmmInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MeteoraDlmmInstruction::ClaimFeeEvent => claim_fee_event::ClaimFeeEvent,
            MeteoraDlmmInstruction::ClaimFee2Event => claim_fee2_event::ClaimFee2Event,
            MeteoraDlmmInstruction::ClaimRewardEvent => claim_reward_event::ClaimRewardEvent,
            MeteoraDlmmInstruction::ClaimReward2Event => claim_reward2_event::ClaimReward2Event,
            MeteoraDlmmInstruction::CompositionFeeEvent => composition_fee_event::CompositionFeeEvent,
            MeteoraDlmmInstruction::DecreasePositionLengthEvent => decrease_position_length_event::DecreasePositionLengthEvent,
            MeteoraDlmmInstruction::DynamicFeeParameterUpdateEvent => dynamic_fee_parameter_update_event::DynamicFeeParameterUpdateEvent,
            MeteoraDlmmInstruction::FeeParameterUpdateEvent => fee_parameter_update_event::FeeParameterUpdateEvent,
            MeteoraDlmmInstruction::FundRewardEvent => fund_reward_event::FundRewardEvent,
            MeteoraDlmmInstruction::GoToABinEvent => go_to_a_bin_event::GoToABinEvent,
            MeteoraDlmmInstruction::IncreaseObservationEvent => increase_observation_event::IncreaseObservationEvent,
            MeteoraDlmmInstruction::IncreasePositionLengthEvent => increase_position_length_event::IncreasePositionLengthEvent,
            MeteoraDlmmInstruction::InitializeRewardEvent => initialize_reward_event::InitializeRewardEvent,
            MeteoraDlmmInstruction::LbPairCreateEvent => lb_pair_create_event::LbPairCreateEvent,
            MeteoraDlmmInstruction::PositionCloseEvent => position_close_event::PositionCloseEvent,
            MeteoraDlmmInstruction::PositionCreateEvent => position_create_event::PositionCreateEvent,
            MeteoraDlmmInstruction::RebalancingEvent => rebalancing_event::RebalancingEvent,
            MeteoraDlmmInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MeteoraDlmmInstruction::SwapEvent => swap_event::SwapEvent,
            MeteoraDlmmInstruction::UpdatePositionLockReleasePointEvent => update_position_lock_release_point_event::UpdatePositionLockReleasePointEvent,
            MeteoraDlmmInstruction::UpdatePositionOperatorEvent => update_position_operator_event::UpdatePositionOperatorEvent,
            MeteoraDlmmInstruction::UpdateRewardDurationEvent => update_reward_duration_event::UpdateRewardDurationEvent,
            MeteoraDlmmInstruction::UpdateRewardFunderEvent => update_reward_funder_event::UpdateRewardFunderEvent,
            MeteoraDlmmInstruction::WithdrawIneligibleRewardEvent => withdraw_ineligible_reward_event::WithdrawIneligibleRewardEvent,
        )
    }
}
