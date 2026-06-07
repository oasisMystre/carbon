use super::RaydiumClmmDecoder;
pub mod close_limit_order;
pub mod close_position;
pub mod close_protocol_position;
pub mod collect_fund_fee;
pub mod collect_personal_fee_event;
pub mod collect_protocol_fee;
pub mod collect_protocol_fee_event;
pub mod collect_remaining_rewards;
pub mod config_change_event;
pub mod create_amm_config;
pub mod create_customizable_pool;
pub mod create_dynamic_fee_config;
pub mod create_operation_account;
pub mod create_personal_position_event;
pub mod create_pool;
pub mod create_support_mint_associated;
pub mod decrease_limit_order;
pub mod decrease_limit_order_event;
pub mod decrease_liquidity;
pub mod decrease_liquidity_event;
pub mod decrease_liquidity_v2;
pub mod increase_limit_order;
pub mod increase_limit_order_event;
pub mod increase_liquidity;
pub mod increase_liquidity_event;
pub mod increase_liquidity_v2;
pub mod initialize_reward;
pub mod liquidity_calculate_event;
pub mod liquidity_change_event;
pub mod open_limit_order;
pub mod open_limit_order_event;
pub mod open_position;
pub mod open_position_v2;
pub mod open_position_with_token22_nft;
pub mod pool_created_event;
pub mod set_reward_params;
pub mod settle_limit_order;
pub mod settle_limit_order_event;
pub mod swap;
pub mod swap_event;
pub mod swap_router_base_in;
pub mod swap_v2;
pub mod transfer_reward_owner;
pub mod update_amm_config;
pub mod update_dynamic_fee_config;
pub mod update_operation_account;
pub mod update_pool_status;
pub mod update_reward_infos;
pub mod update_reward_infos_event;

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
pub enum RaydiumClmmInstruction {
    CloseLimitOrder(close_limit_order::CloseLimitOrder),
    ClosePosition(close_position::ClosePosition),
    CloseProtocolPosition(close_protocol_position::CloseProtocolPosition),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectRemainingRewards(collect_remaining_rewards::CollectRemainingRewards),
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    CreateCustomizablePool(create_customizable_pool::CreateCustomizablePool),
    CreateDynamicFeeConfig(create_dynamic_fee_config::CreateDynamicFeeConfig),
    CreateOperationAccount(create_operation_account::CreateOperationAccount),
    CreatePool(create_pool::CreatePool),
    CreateSupportMintAssociated(create_support_mint_associated::CreateSupportMintAssociated),
    DecreaseLimitOrder(decrease_limit_order::DecreaseLimitOrder),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    IncreaseLimitOrder(increase_limit_order::IncreaseLimitOrder),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializeReward(initialize_reward::InitializeReward),
    OpenLimitOrder(open_limit_order::OpenLimitOrder),
    OpenPosition(open_position::OpenPosition),
    OpenPositionV2(open_position_v2::OpenPositionV2),
    OpenPositionWithToken22Nft(open_position_with_token22_nft::OpenPositionWithToken22Nft),
    SetRewardParams(set_reward_params::SetRewardParams),
    SettleLimitOrder(settle_limit_order::SettleLimitOrder),
    Swap(swap::Swap),
    SwapRouterBaseIn(swap_router_base_in::SwapRouterBaseIn),
    SwapV2(swap_v2::SwapV2),
    TransferRewardOwner(transfer_reward_owner::TransferRewardOwner),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    UpdateDynamicFeeConfig(update_dynamic_fee_config::UpdateDynamicFeeConfig),
    UpdateOperationAccount(update_operation_account::UpdateOperationAccount),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    UpdateRewardInfos(update_reward_infos::UpdateRewardInfos),
    CollectPersonalFeeEvent(collect_personal_fee_event::CollectPersonalFeeEvent),
    CollectProtocolFeeEvent(collect_protocol_fee_event::CollectProtocolFeeEvent),
    ConfigChangeEvent(config_change_event::ConfigChangeEvent),
    CreatePersonalPositionEvent(create_personal_position_event::CreatePersonalPositionEvent),
    DecreaseLimitOrderEvent(decrease_limit_order_event::DecreaseLimitOrderEvent),
    DecreaseLiquidityEvent(decrease_liquidity_event::DecreaseLiquidityEvent),
    IncreaseLimitOrderEvent(increase_limit_order_event::IncreaseLimitOrderEvent),
    IncreaseLiquidityEvent(increase_liquidity_event::IncreaseLiquidityEvent),
    LiquidityCalculateEvent(liquidity_calculate_event::LiquidityCalculateEvent),
    LiquidityChangeEvent(liquidity_change_event::LiquidityChangeEvent),
    OpenLimitOrderEvent(open_limit_order_event::OpenLimitOrderEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    SettleLimitOrderEvent(settle_limit_order_event::SettleLimitOrderEvent),
    SwapEvent(swap_event::SwapEvent),
    UpdateRewardInfosEvent(update_reward_infos_event::UpdateRewardInfosEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for RaydiumClmmDecoder {
    type InstructionType = RaydiumClmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            RaydiumClmmInstruction::CloseLimitOrder => close_limit_order::CloseLimitOrder,
            RaydiumClmmInstruction::ClosePosition => close_position::ClosePosition,
            RaydiumClmmInstruction::CloseProtocolPosition => close_protocol_position::CloseProtocolPosition,
            RaydiumClmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            RaydiumClmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            RaydiumClmmInstruction::CollectRemainingRewards => collect_remaining_rewards::CollectRemainingRewards,
            RaydiumClmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            RaydiumClmmInstruction::CreateCustomizablePool => create_customizable_pool::CreateCustomizablePool,
            RaydiumClmmInstruction::CreateDynamicFeeConfig => create_dynamic_fee_config::CreateDynamicFeeConfig,
            RaydiumClmmInstruction::CreateOperationAccount => create_operation_account::CreateOperationAccount,
            RaydiumClmmInstruction::CreatePool => create_pool::CreatePool,
            RaydiumClmmInstruction::CreateSupportMintAssociated => create_support_mint_associated::CreateSupportMintAssociated,
            RaydiumClmmInstruction::DecreaseLimitOrder => decrease_limit_order::DecreaseLimitOrder,
            RaydiumClmmInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            RaydiumClmmInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            RaydiumClmmInstruction::IncreaseLimitOrder => increase_limit_order::IncreaseLimitOrder,
            RaydiumClmmInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            RaydiumClmmInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            RaydiumClmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            RaydiumClmmInstruction::OpenLimitOrder => open_limit_order::OpenLimitOrder,
            RaydiumClmmInstruction::OpenPosition => open_position::OpenPosition,
            RaydiumClmmInstruction::OpenPositionV2 => open_position_v2::OpenPositionV2,
            RaydiumClmmInstruction::OpenPositionWithToken22Nft => open_position_with_token22_nft::OpenPositionWithToken22Nft,
            RaydiumClmmInstruction::SetRewardParams => set_reward_params::SetRewardParams,
            RaydiumClmmInstruction::SettleLimitOrder => settle_limit_order::SettleLimitOrder,
            RaydiumClmmInstruction::Swap => swap::Swap,
            RaydiumClmmInstruction::SwapRouterBaseIn => swap_router_base_in::SwapRouterBaseIn,
            RaydiumClmmInstruction::SwapV2 => swap_v2::SwapV2,
            RaydiumClmmInstruction::TransferRewardOwner => transfer_reward_owner::TransferRewardOwner,
            RaydiumClmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            RaydiumClmmInstruction::UpdateDynamicFeeConfig => update_dynamic_fee_config::UpdateDynamicFeeConfig,
            RaydiumClmmInstruction::UpdateOperationAccount => update_operation_account::UpdateOperationAccount,
            RaydiumClmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            RaydiumClmmInstruction::UpdateRewardInfos => update_reward_infos::UpdateRewardInfos,
            RaydiumClmmInstruction::CollectPersonalFeeEvent => collect_personal_fee_event::CollectPersonalFeeEvent,
            RaydiumClmmInstruction::CollectProtocolFeeEvent => collect_protocol_fee_event::CollectProtocolFeeEvent,
            RaydiumClmmInstruction::ConfigChangeEvent => config_change_event::ConfigChangeEvent,
            RaydiumClmmInstruction::CreatePersonalPositionEvent => create_personal_position_event::CreatePersonalPositionEvent,
            RaydiumClmmInstruction::DecreaseLimitOrderEvent => decrease_limit_order_event::DecreaseLimitOrderEvent,
            RaydiumClmmInstruction::DecreaseLiquidityEvent => decrease_liquidity_event::DecreaseLiquidityEvent,
            RaydiumClmmInstruction::IncreaseLimitOrderEvent => increase_limit_order_event::IncreaseLimitOrderEvent,
            RaydiumClmmInstruction::IncreaseLiquidityEvent => increase_liquidity_event::IncreaseLiquidityEvent,
            RaydiumClmmInstruction::LiquidityCalculateEvent => liquidity_calculate_event::LiquidityCalculateEvent,
            RaydiumClmmInstruction::LiquidityChangeEvent => liquidity_change_event::LiquidityChangeEvent,
            RaydiumClmmInstruction::OpenLimitOrderEvent => open_limit_order_event::OpenLimitOrderEvent,
            RaydiumClmmInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            RaydiumClmmInstruction::SettleLimitOrderEvent => settle_limit_order_event::SettleLimitOrderEvent,
            RaydiumClmmInstruction::SwapEvent => swap_event::SwapEvent,
            RaydiumClmmInstruction::UpdateRewardInfosEvent => update_reward_infos_event::UpdateRewardInfosEvent,
        )
    }
}
