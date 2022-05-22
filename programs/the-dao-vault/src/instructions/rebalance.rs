use std::{convert::TryFrom, ops::Deref};

use boolinator::Boolinator;
use strum::IntoEnumIterator;

use anchor_lang::prelude::*;
use port_anchor_adaptor::PortReserve;
use solana_maths::Rate;

use crate::{
    adapters::SolendReserve, asset_container::AssetContainer, errors::ErrorCode,
    impl_provider_index, reserves::Reserves, state::*,
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

impl TryFrom<&Rebalance<'_>> for AssetContainer<Reserves> {
    type Error = ProgramError;

    fn try_from(r: &Rebalance<'_>) -> std::result::Result<AssetContainer<Reserves>, Self::Error> {
        let solend = Some(Reserves::Solend(r.solend_reserve.deref().deref().clone()));
        let port = Some(Reserves::Port(r.port_reserve.deref().deref().clone()));

        Ok(AssetContainer {
            inner: [solend, port],
        })
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct StrategyWeightsArg {
    solend: u16,
    port: u16,
}
impl_provider_index!(StrategyWeightsArg, u16);

impl From<StrategyWeightsArg> for AssetContainer<Rate> {
    fn from(s: StrategyWeightsArg) -> Self {
        Provider::iter().fold(Self::default(), |mut acc, provider| {
            acc[provider] = Rate::from_bips(s[provider] as u64);
            acc
        })
    }
}

pub fn handler(ctx: Context<Rebalance>, proposed_weights_arg: StrategyWeightsArg) -> Result<()> {
    #[cfg(feature = "debug")]
    msg!("Rebalancing");

    let vault_value = ctx.accounts.vault.value.value;
    let slot = Clock::get()?.slot;

    let assets = Box::new(AssetContainer::try_from(&*ctx.accounts)?);
    let strategy_weights = assets.calculate_weights(
        ctx.accounts.vault.config.strategy_type,
        ctx.accounts.vault.config.allocation_cap_pct,
    )?;

    AssetContainer::<u64>::try_from_weights(&strategy_weights, vault_value).and_then(
        |strategy_allocations| match ctx.accounts.vault.config.rebalance_mode {
            RebalanceMode::ProofChecker => {
                let proposed_weights = AssetContainer::<Rate>::from(proposed_weights_arg);
                let proposed_allocations = AssetContainer::<u64>::try_from_weights(&strategy_weights, vault_value)?;

                #[cfg(feature = "debug")]
                msg!("Running as proof checker with proposed weights: {:?}", proposed_weights.innder);

                // Check that proposed weights meet necessary constraints
                proposed_weights.verify_weights(ctx.accounts.vault.config.allocation_cap_pct)?;
                
                let proposed_apr = assets.get_apr(&proposed_weights, &proposed_allocations)?;
                let proof_apr = assets.get_apr(&strategy_weights, &strategy_allocations)?;

                #[cfg(feature = "debug")]
                msg!("Proposed APR: {:?}\nProof APR: {:?}", proposed_apr, proof_apr);

                (proposed_apr >= proof_apr).as_result(
                    proposed_allocations,
                    ErrorCode::RebalanceProofCheckFailed.into()
                )
            }
            RebalanceMode::Calculator => {
                #[cfg(feature = "debug")]
                msg!("Running as calculator");

                Ok(strategy_allocations)
            }
        }
    ).map(|final_allocations_container| {
        let final_allocations = Allocations::from_container(final_allocations_container, slot);

        #[cfg(feature = "debug")]
        msg!("Final allocations {:?}", final_allocations);

        emit!(RebalanceEvent {
            vault: ctx.accounts.vault.key()
        });
        emit!(RebalanceDataEvent::from(&final_allocations));



        ctx.accounts.vault.target_allocations = final_allocations;
    })
}
