use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb46945505f32496c")]
pub struct ForIdlTypeGenerationDoNotCall {
    pub ix: DummyIx,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ForIdlTypeGenerationDoNotCallInstructionAccounts {
    pub dummy_zc_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForIdlTypeGenerationDoNotCall {
    type ArrangedAccounts = ForIdlTypeGenerationDoNotCallInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [dummy_zc_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ForIdlTypeGenerationDoNotCallInstructionAccounts {
            dummy_zc_account: dummy_zc_account.pubkey,
        })
    }
}
