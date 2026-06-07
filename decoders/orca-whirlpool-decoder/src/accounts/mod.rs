use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::WhirlpoolDecoder;
pub mod adaptive_fee_tier;
pub mod dynamic_tick_array;
pub mod fee_tier;
pub mod lock_config;
pub mod oracle;
pub mod position;
pub mod position_bundle;
pub mod tick_array;
pub mod token_badge;
pub mod whirlpool;
pub mod whirlpools_config;
pub mod whirlpools_config_extension;

pub enum WhirlpoolAccount {
    AdaptiveFeeTier(adaptive_fee_tier::AdaptiveFeeTier),
    DynamicTickArray(dynamic_tick_array::DynamicTickArray),
    FeeTier(fee_tier::FeeTier),
    LockConfig(lock_config::LockConfig),
    Oracle(oracle::Oracle),
    Position(position::Position),
    PositionBundle(position_bundle::PositionBundle),
    TickArray(tick_array::TickArray),
    TokenBadge(token_badge::TokenBadge),
    Whirlpool(whirlpool::Whirlpool),
    WhirlpoolsConfig(whirlpools_config::WhirlpoolsConfig),
    WhirlpoolsConfigExtension(whirlpools_config_extension::WhirlpoolsConfigExtension),
}

impl<'a> AccountDecoder<'a> for WhirlpoolDecoder {
    type AccountType = WhirlpoolAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            adaptive_fee_tier::AdaptiveFeeTier::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::AdaptiveFeeTier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            dynamic_tick_array::DynamicTickArray::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::DynamicTickArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee_tier::FeeTier::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::FeeTier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lock_config::LockConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::LockConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = oracle::Oracle::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Oracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            position_bundle::PositionBundle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::PositionBundle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = tick_array::TickArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::TickArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = whirlpool::Whirlpool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Whirlpool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpools_config::WhirlpoolsConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::WhirlpoolsConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpools_config_extension::WhirlpoolsConfigExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::WhirlpoolsConfigExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
