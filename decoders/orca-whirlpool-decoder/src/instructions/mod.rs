use super::OrcaWhirlpoolDecoder;
pub mod close_bundled_position;
pub mod close_position;
pub mod close_position_with_token_extensions;
pub mod collect_fees;
pub mod collect_fees_v2;
pub mod collect_protocol_fees;
pub mod collect_protocol_fees_v2;
pub mod collect_reward;
pub mod collect_reward_v2;
pub mod decrease_liquidity;
pub mod decrease_liquidity_v2;
pub mod delete_position_bundle;
pub mod delete_token_badge;
pub mod idl_include;
pub mod increase_liquidity;
pub mod increase_liquidity_by_token_amounts_v2;
pub mod increase_liquidity_v2;
pub mod initialize_adaptive_fee_tier;
pub mod initialize_config;
pub mod initialize_config_extension;
pub mod initialize_dynamic_tick_array;
pub mod initialize_fee_tier;
pub mod initialize_pool;
pub mod initialize_pool_v2;
pub mod initialize_pool_with_adaptive_fee;
pub mod initialize_position_bundle;
pub mod initialize_position_bundle_with_metadata;
pub mod initialize_reward;
pub mod initialize_reward_v2;
pub mod initialize_tick_array;
pub mod initialize_token_badge;
pub mod liquidity_decreased_event;
pub mod liquidity_increased_event;
pub mod liquidity_repositioned_event;
pub mod lock_position;
pub mod migrate_repurpose_reward_authority_space;
pub mod open_bundled_position;
pub mod open_position;
pub mod open_position_with_metadata;
pub mod open_position_with_token_extensions;
pub mod pool_initialized_event;
pub mod position_opened_event;
pub mod reposition_liquidity_v2;
pub mod reset_position_range;
pub mod set_adaptive_fee_constants;
pub mod set_collect_protocol_fees_authority;
pub mod set_config_extension_authority;
pub mod set_config_feature_flag;
pub mod set_default_base_fee_rate;
pub mod set_default_fee_rate;
pub mod set_default_protocol_fee_rate;
pub mod set_delegated_fee_authority;
pub mod set_fee_authority;
pub mod set_fee_rate;
pub mod set_fee_rate_by_delegated_fee_authority;
pub mod set_initialize_pool_authority;
pub mod set_preset_adaptive_fee_constants;
pub mod set_protocol_fee_rate;
pub mod set_reward_authority;
pub mod set_reward_authority_by_super_authority;
pub mod set_reward_emissions;
pub mod set_reward_emissions_super_authority;
pub mod set_reward_emissions_v2;
pub mod set_token_badge_attribute;
pub mod set_token_badge_authority;
pub mod swap;
pub mod swap_v2;
pub mod traded_event;
pub mod transfer_locked_position;
pub mod two_hop_swap;
pub mod two_hop_swap_v2;
pub mod update_fees_and_rewards;

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
pub enum OrcaWhirlpoolInstruction {
    CloseBundledPosition(close_bundled_position::CloseBundledPosition),
    ClosePosition(close_position::ClosePosition),
    ClosePositionWithTokenExtensions(
        close_position_with_token_extensions::ClosePositionWithTokenExtensions,
    ),
    CollectFees(collect_fees::CollectFees),
    CollectFeesV2(collect_fees_v2::CollectFeesV2),
    CollectProtocolFees(collect_protocol_fees::CollectProtocolFees),
    CollectProtocolFeesV2(collect_protocol_fees_v2::CollectProtocolFeesV2),
    CollectReward(collect_reward::CollectReward),
    CollectRewardV2(collect_reward_v2::CollectRewardV2),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    DeletePositionBundle(delete_position_bundle::DeletePositionBundle),
    DeleteTokenBadge(delete_token_badge::DeleteTokenBadge),
    IdlInclude(idl_include::IdlInclude),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityByTokenAmountsV2(
        increase_liquidity_by_token_amounts_v2::IncreaseLiquidityByTokenAmountsV2,
    ),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializeAdaptiveFeeTier(initialize_adaptive_fee_tier::InitializeAdaptiveFeeTier),
    InitializeConfig(initialize_config::InitializeConfig),
    InitializeConfigExtension(initialize_config_extension::InitializeConfigExtension),
    InitializeDynamicTickArray(initialize_dynamic_tick_array::InitializeDynamicTickArray),
    InitializeFeeTier(initialize_fee_tier::InitializeFeeTier),
    InitializePool(initialize_pool::InitializePool),
    InitializePoolV2(initialize_pool_v2::InitializePoolV2),
    InitializePoolWithAdaptiveFee(initialize_pool_with_adaptive_fee::InitializePoolWithAdaptiveFee),
    InitializePositionBundle(initialize_position_bundle::InitializePositionBundle),
    InitializePositionBundleWithMetadata(
        initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
    ),
    InitializeReward(initialize_reward::InitializeReward),
    InitializeRewardV2(initialize_reward_v2::InitializeRewardV2),
    InitializeTickArray(initialize_tick_array::InitializeTickArray),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    LockPosition(lock_position::LockPosition),
    MigrateRepurposeRewardAuthoritySpace(
        migrate_repurpose_reward_authority_space::MigrateRepurposeRewardAuthoritySpace,
    ),
    OpenBundledPosition(open_bundled_position::OpenBundledPosition),
    OpenPosition(open_position::OpenPosition),
    OpenPositionWithMetadata(open_position_with_metadata::OpenPositionWithMetadata),
    OpenPositionWithTokenExtensions(
        open_position_with_token_extensions::OpenPositionWithTokenExtensions,
    ),
    RepositionLiquidityV2(reposition_liquidity_v2::RepositionLiquidityV2),
    ResetPositionRange(reset_position_range::ResetPositionRange),
    SetAdaptiveFeeConstants(set_adaptive_fee_constants::SetAdaptiveFeeConstants),
    SetCollectProtocolFeesAuthority(
        set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
    ),
    SetConfigExtensionAuthority(set_config_extension_authority::SetConfigExtensionAuthority),
    SetConfigFeatureFlag(set_config_feature_flag::SetConfigFeatureFlag),
    SetDefaultBaseFeeRate(set_default_base_fee_rate::SetDefaultBaseFeeRate),
    SetDefaultFeeRate(set_default_fee_rate::SetDefaultFeeRate),
    SetDefaultProtocolFeeRate(set_default_protocol_fee_rate::SetDefaultProtocolFeeRate),
    SetDelegatedFeeAuthority(set_delegated_fee_authority::SetDelegatedFeeAuthority),
    SetFeeAuthority(set_fee_authority::SetFeeAuthority),
    SetFeeRate(set_fee_rate::SetFeeRate),
    SetFeeRateByDelegatedFeeAuthority(
        set_fee_rate_by_delegated_fee_authority::SetFeeRateByDelegatedFeeAuthority,
    ),
    SetInitializePoolAuthority(set_initialize_pool_authority::SetInitializePoolAuthority),
    SetPresetAdaptiveFeeConstants(set_preset_adaptive_fee_constants::SetPresetAdaptiveFeeConstants),
    SetProtocolFeeRate(set_protocol_fee_rate::SetProtocolFeeRate),
    SetRewardAuthority(set_reward_authority::SetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(
        set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
    ),
    SetRewardEmissions(set_reward_emissions::SetRewardEmissions),
    SetRewardEmissionsSuperAuthority(
        set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
    ),
    SetRewardEmissionsV2(set_reward_emissions_v2::SetRewardEmissionsV2),
    SetTokenBadgeAttribute(set_token_badge_attribute::SetTokenBadgeAttribute),
    SetTokenBadgeAuthority(set_token_badge_authority::SetTokenBadgeAuthority),
    Swap(swap::Swap),
    SwapV2(swap_v2::SwapV2),
    TransferLockedPosition(transfer_locked_position::TransferLockedPosition),
    TwoHopSwap(two_hop_swap::TwoHopSwap),
    TwoHopSwapV2(two_hop_swap_v2::TwoHopSwapV2),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    LiquidityDecreasedEvent(liquidity_decreased_event::LiquidityDecreasedEvent),
    LiquidityIncreasedEvent(liquidity_increased_event::LiquidityIncreasedEvent),
    LiquidityRepositionedEvent(liquidity_repositioned_event::LiquidityRepositionedEvent),
    PoolInitializedEvent(pool_initialized_event::PoolInitializedEvent),
    PositionOpenedEvent(position_opened_event::PositionOpenedEvent),
    TradedEvent(traded_event::TradedEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for OrcaWhirlpoolDecoder {
    type InstructionType = OrcaWhirlpoolInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            OrcaWhirlpoolInstruction::CloseBundledPosition => close_bundled_position::CloseBundledPosition,
            OrcaWhirlpoolInstruction::ClosePosition => close_position::ClosePosition,
            OrcaWhirlpoolInstruction::ClosePositionWithTokenExtensions => close_position_with_token_extensions::ClosePositionWithTokenExtensions,
            OrcaWhirlpoolInstruction::CollectFees => collect_fees::CollectFees,
            OrcaWhirlpoolInstruction::CollectFeesV2 => collect_fees_v2::CollectFeesV2,
            OrcaWhirlpoolInstruction::CollectProtocolFees => collect_protocol_fees::CollectProtocolFees,
            OrcaWhirlpoolInstruction::CollectProtocolFeesV2 => collect_protocol_fees_v2::CollectProtocolFeesV2,
            OrcaWhirlpoolInstruction::CollectReward => collect_reward::CollectReward,
            OrcaWhirlpoolInstruction::CollectRewardV2 => collect_reward_v2::CollectRewardV2,
            OrcaWhirlpoolInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            OrcaWhirlpoolInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            OrcaWhirlpoolInstruction::DeletePositionBundle => delete_position_bundle::DeletePositionBundle,
            OrcaWhirlpoolInstruction::DeleteTokenBadge => delete_token_badge::DeleteTokenBadge,
            OrcaWhirlpoolInstruction::IdlInclude => idl_include::IdlInclude,
            OrcaWhirlpoolInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            OrcaWhirlpoolInstruction::IncreaseLiquidityByTokenAmountsV2 => increase_liquidity_by_token_amounts_v2::IncreaseLiquidityByTokenAmountsV2,
            OrcaWhirlpoolInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            OrcaWhirlpoolInstruction::InitializeAdaptiveFeeTier => initialize_adaptive_fee_tier::InitializeAdaptiveFeeTier,
            OrcaWhirlpoolInstruction::InitializeConfig => initialize_config::InitializeConfig,
            OrcaWhirlpoolInstruction::InitializeConfigExtension => initialize_config_extension::InitializeConfigExtension,
            OrcaWhirlpoolInstruction::InitializeDynamicTickArray => initialize_dynamic_tick_array::InitializeDynamicTickArray,
            OrcaWhirlpoolInstruction::InitializeFeeTier => initialize_fee_tier::InitializeFeeTier,
            OrcaWhirlpoolInstruction::InitializePool => initialize_pool::InitializePool,
            OrcaWhirlpoolInstruction::InitializePoolV2 => initialize_pool_v2::InitializePoolV2,
            OrcaWhirlpoolInstruction::InitializePoolWithAdaptiveFee => initialize_pool_with_adaptive_fee::InitializePoolWithAdaptiveFee,
            OrcaWhirlpoolInstruction::InitializePositionBundle => initialize_position_bundle::InitializePositionBundle,
            OrcaWhirlpoolInstruction::InitializePositionBundleWithMetadata => initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
            OrcaWhirlpoolInstruction::InitializeReward => initialize_reward::InitializeReward,
            OrcaWhirlpoolInstruction::InitializeRewardV2 => initialize_reward_v2::InitializeRewardV2,
            OrcaWhirlpoolInstruction::InitializeTickArray => initialize_tick_array::InitializeTickArray,
            OrcaWhirlpoolInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            OrcaWhirlpoolInstruction::LockPosition => lock_position::LockPosition,
            OrcaWhirlpoolInstruction::MigrateRepurposeRewardAuthoritySpace => migrate_repurpose_reward_authority_space::MigrateRepurposeRewardAuthoritySpace,
            OrcaWhirlpoolInstruction::OpenBundledPosition => open_bundled_position::OpenBundledPosition,
            OrcaWhirlpoolInstruction::OpenPosition => open_position::OpenPosition,
            OrcaWhirlpoolInstruction::OpenPositionWithMetadata => open_position_with_metadata::OpenPositionWithMetadata,
            OrcaWhirlpoolInstruction::OpenPositionWithTokenExtensions => open_position_with_token_extensions::OpenPositionWithTokenExtensions,
            OrcaWhirlpoolInstruction::RepositionLiquidityV2 => reposition_liquidity_v2::RepositionLiquidityV2,
            OrcaWhirlpoolInstruction::ResetPositionRange => reset_position_range::ResetPositionRange,
            OrcaWhirlpoolInstruction::SetAdaptiveFeeConstants => set_adaptive_fee_constants::SetAdaptiveFeeConstants,
            OrcaWhirlpoolInstruction::SetCollectProtocolFeesAuthority => set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
            OrcaWhirlpoolInstruction::SetConfigExtensionAuthority => set_config_extension_authority::SetConfigExtensionAuthority,
            OrcaWhirlpoolInstruction::SetConfigFeatureFlag => set_config_feature_flag::SetConfigFeatureFlag,
            OrcaWhirlpoolInstruction::SetDefaultBaseFeeRate => set_default_base_fee_rate::SetDefaultBaseFeeRate,
            OrcaWhirlpoolInstruction::SetDefaultFeeRate => set_default_fee_rate::SetDefaultFeeRate,
            OrcaWhirlpoolInstruction::SetDefaultProtocolFeeRate => set_default_protocol_fee_rate::SetDefaultProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetDelegatedFeeAuthority => set_delegated_fee_authority::SetDelegatedFeeAuthority,
            OrcaWhirlpoolInstruction::SetFeeAuthority => set_fee_authority::SetFeeAuthority,
            OrcaWhirlpoolInstruction::SetFeeRate => set_fee_rate::SetFeeRate,
            OrcaWhirlpoolInstruction::SetFeeRateByDelegatedFeeAuthority => set_fee_rate_by_delegated_fee_authority::SetFeeRateByDelegatedFeeAuthority,
            OrcaWhirlpoolInstruction::SetInitializePoolAuthority => set_initialize_pool_authority::SetInitializePoolAuthority,
            OrcaWhirlpoolInstruction::SetPresetAdaptiveFeeConstants => set_preset_adaptive_fee_constants::SetPresetAdaptiveFeeConstants,
            OrcaWhirlpoolInstruction::SetProtocolFeeRate => set_protocol_fee_rate::SetProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetRewardAuthority => set_reward_authority::SetRewardAuthority,
            OrcaWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority => set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
            OrcaWhirlpoolInstruction::SetRewardEmissions => set_reward_emissions::SetRewardEmissions,
            OrcaWhirlpoolInstruction::SetRewardEmissionsSuperAuthority => set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
            OrcaWhirlpoolInstruction::SetRewardEmissionsV2 => set_reward_emissions_v2::SetRewardEmissionsV2,
            OrcaWhirlpoolInstruction::SetTokenBadgeAttribute => set_token_badge_attribute::SetTokenBadgeAttribute,
            OrcaWhirlpoolInstruction::SetTokenBadgeAuthority => set_token_badge_authority::SetTokenBadgeAuthority,
            OrcaWhirlpoolInstruction::Swap => swap::Swap,
            OrcaWhirlpoolInstruction::SwapV2 => swap_v2::SwapV2,
            OrcaWhirlpoolInstruction::TransferLockedPosition => transfer_locked_position::TransferLockedPosition,
            OrcaWhirlpoolInstruction::TwoHopSwap => two_hop_swap::TwoHopSwap,
            OrcaWhirlpoolInstruction::TwoHopSwapV2 => two_hop_swap_v2::TwoHopSwapV2,
            OrcaWhirlpoolInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            OrcaWhirlpoolInstruction::LiquidityDecreasedEvent => liquidity_decreased_event::LiquidityDecreasedEvent,
            OrcaWhirlpoolInstruction::LiquidityIncreasedEvent => liquidity_increased_event::LiquidityIncreasedEvent,
            OrcaWhirlpoolInstruction::LiquidityRepositionedEvent => liquidity_repositioned_event::LiquidityRepositionedEvent,
            OrcaWhirlpoolInstruction::PoolInitializedEvent => pool_initialized_event::PoolInitializedEvent,
            OrcaWhirlpoolInstruction::PositionOpenedEvent => position_opened_event::PositionOpenedEvent,
            OrcaWhirlpoolInstruction::TradedEvent => traded_event::TradedEvent,
        )
    }
}
