use std::{convert::TryFrom, ops::Deref};

use boolinator::Boolinator;
use strum::IntoEnumIterator;

use anchor_lang::prelude::*;
use port_anchor_adaptor::PortReserve;
use solana_maths::Rate;

use crate::{
    adapters::SolendReserve,
    asset_container::AssetContainer,
    errors::ErrorCode,
    impl_provider_index,
    reserves::{Reserves},
    state::*,
};

#[event]
pub struct RebalanceEvent {
    vault: Pubkey,
}

/// Used by the SDK to figure out the order in which reconcile TXs should be sent
#[event]
#[derive(Default)]
pub struct RebalanceDataEvent {
    solend: u64,
    port: u64,
}
impl_provider_index!(RebalanceDataEvent, u64);

impl From<&Allocations> for RebalanceDataEvent {
    fn from(allocations: &Allocations) -> Self {
        Provider::iter().fold(Self::default(), |mut acc, provider| {
            acc[provider] = allocations[provider].value;
            acc
        })
    }
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    /// Vault state account
    /// Checks that the refresh has been called in the small slot
    /// Chekcs that the accounts passed in are correct
    #[account(mut, constraint = !vault.value.last_update.is_stale(clock.slot)? @ErrorCode::VaultIsNotRefreshed, has_one = solend_reserve, has_one = port_reserve)]
    pub vault: Box<Account<'info, Vault>>,

    pub solend_reserve: Box<Account<'info, SolendReserve>>,

    pub port_reserve: Box<Account<'info, PortReserve>>,

    pub clock: Sysvar<'info, Clock>,
}
