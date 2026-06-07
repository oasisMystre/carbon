use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4db84ad67056f1c7")]
pub struct OpenPositionV2 {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct OpenPositionV2InstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub position_nft_owner: solana_pubkey::Pubkey,
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub metadata_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub token_account_0: solana_pubkey::Pubkey,
    pub token_account_1: solana_pubkey::Pubkey,
    pub token_vault_0: solana_pubkey::Pubkey,
    pub token_vault_1: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
    pub vault_0_mint: solana_pubkey::Pubkey,
    pub vault_1_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionV2 {
    type ArrangedAccounts = OpenPositionV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let position_nft_owner = next_account(&mut iter)?;
        let position_nft_mint = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let metadata_account = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let protocol_position = next_account(&mut iter)?;
        let tick_array_lower = next_account(&mut iter)?;
        let tick_array_upper = next_account(&mut iter)?;
        let personal_position = next_account(&mut iter)?;
        let token_account_0 = next_account(&mut iter)?;
        let token_account_1 = next_account(&mut iter)?;
        let token_vault_0 = next_account(&mut iter)?;
        let token_vault_1 = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;

        Some(OpenPositionV2InstructionAccounts {
            payer,
            position_nft_owner,
            position_nft_mint,
            position_nft_account,
            metadata_account,
            pool_state,
            protocol_position,
            tick_array_lower,
            tick_array_upper,
            personal_position,
            token_account_0,
            token_account_1,
            token_vault_0,
            token_vault_1,
            rent,
            system_program,
            token_program,
            associated_token_program,
            metadata_program,
            token_program_2022,
            vault_0_mint,
            vault_1_mint,
        })
    }
}
