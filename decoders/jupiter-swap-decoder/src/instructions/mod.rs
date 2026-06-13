



use super::JupiterSwapDecoder;
pub mod route;
pub mod route_with_token_ledger;
pub mod exact_out_route;
pub mod shared_accounts_route;
pub mod shared_accounts_route_with_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod set_token_ledger;
pub mod create_open_orders;
pub mod create_token_account;
pub mod create_program_open_orders;
pub mod claim;
pub mod claim_token;
pub mod create_token_ledger;
pub mod swap_event;
pub mod fee_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum JupiterSwapInstruction {
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    ExactOutRoute(exact_out_route::ExactOutRoute),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger),
    SharedAccountsExactOutRoute(shared_accounts_exact_out_route::SharedAccountsExactOutRoute),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
    CreateTokenAccount(create_token_account::CreateTokenAccount),
    CreateProgramOpenOrders(create_program_open_orders::CreateProgramOpenOrders),
    Claim(claim::Claim),
    ClaimToken(claim_token::ClaimToken),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    SwapEvent(swap_event::SwapEvent),
    FeeEvent(fee_event::FeeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterSwapDecoder {
    type InstructionType = JupiterSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterSwapInstruction::Route => route::Route,
            JupiterSwapInstruction::RouteWithTokenLedger => route_with_token_ledger::RouteWithTokenLedger,
            JupiterSwapInstruction::ExactOutRoute => exact_out_route::ExactOutRoute,
            JupiterSwapInstruction::SharedAccountsRoute => shared_accounts_route::SharedAccountsRoute,
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger => shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
            JupiterSwapInstruction::SharedAccountsExactOutRoute => shared_accounts_exact_out_route::SharedAccountsExactOutRoute,
            JupiterSwapInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterSwapInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
            JupiterSwapInstruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            JupiterSwapInstruction::CreateProgramOpenOrders => create_program_open_orders::CreateProgramOpenOrders,
            JupiterSwapInstruction::Claim => claim::Claim,
            JupiterSwapInstruction::ClaimToken => claim_token::ClaimToken,
            JupiterSwapInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            JupiterSwapInstruction::SwapEvent => swap_event::SwapEvent,
            JupiterSwapInstruction::FeeEvent => fee_event::FeeEvent,
        )
    }
}