use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::MeteoraDlmmDecoder;
pub mod bin_array;
pub mod bin_array_bitmap_extension;
pub mod claim_fee_operator;
pub mod dummy_zc_account;
pub mod lb_pair;
pub mod oracle;
pub mod position;
pub mod position_v2;
pub mod preset_parameter;
pub mod preset_parameter2;
pub mod token_badge;

pub enum MeteoraDlmmAccount {
    BinArray(bin_array::BinArray),
    BinArrayBitmapExtension(bin_array_bitmap_extension::BinArrayBitmapExtension),
    ClaimFeeOperator(claim_fee_operator::ClaimFeeOperator),
    DummyZcAccount(dummy_zc_account::DummyZcAccount),
    LbPair(lb_pair::LbPair),
    Oracle(oracle::Oracle),
    Position(position::Position),
    PositionV2(position_v2::PositionV2),
    PresetParameter(preset_parameter::PresetParameter),
    PresetParameter2(preset_parameter2::PresetParameter2),
    TokenBadge(token_badge::TokenBadge),
}

impl<'a> AccountDecoder<'a> for MeteoraDlmmDecoder {
    type AccountType = MeteoraDlmmAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = bin_array::BinArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            bin_array_bitmap_extension::BinArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::BinArrayBitmapExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            claim_fee_operator::ClaimFeeOperator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::ClaimFeeOperator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            dummy_zc_account::DummyZcAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::DummyZcAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lb_pair::LbPair::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::LbPair(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = oracle::Oracle::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Oracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position_v2::PositionV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PositionV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            preset_parameter::PresetParameter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PresetParameter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            preset_parameter2::PresetParameter2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::PresetParameter2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MeteoraDlmmAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
