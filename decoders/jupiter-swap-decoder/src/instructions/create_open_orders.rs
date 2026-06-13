

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe5c2d4ac080a8693")]
pub struct CreateOpenOrders{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOpenOrdersInstructionAccounts {
    pub open_orders: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrders {
    type ArrangedAccounts = CreateOpenOrdersInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let open_orders = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let dex_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let market = next_account(&mut iter)?;

        Some(CreateOpenOrdersInstructionAccounts {
            open_orders,
            payer,
            dex_program,
            system_program,
            rent,
            market,
        })
    }
}