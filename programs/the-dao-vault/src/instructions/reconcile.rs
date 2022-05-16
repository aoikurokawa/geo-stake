use std::cmp;

use anchor_lang::prelude::*;
use boolinator::Boolinator;

use crate::{
    errors::ErrorCode,
    reserves::Provider,
    state::{Vault, VaultFlags},
};

const MAX_SLOTS_SINCE_ALLOC_UPDATE: u64 = 100;

pub trait LendingMarket {
    fn deposit(&self, amount: u64) -> Result<()>;
    fn redeem(&self, amount: u64) -> Result<()>;

    // TODO separate these fns into Exchange struct
    // OR Amount struct like Jet which handles coversions implicity
    fn convert_amount_reserve_to_lp(&self, amount: u64) -> Result<u64>;
    fn convert_amount_lp_to_reserve(&self, amount: u64) -> Result<u64>;

    fn reserve_tokens_in_vault(&self) -> u64;
    fn lp_tokens_in_vault(&self) -> u64;

    fn provider(&self) -> Provider;
}

pub trait HasVault {
    fn vault(&self) -> &Vault;
    fn vault_mut(&mut self) -> &mut Vault;
}

// TODO make this a custom derive promacro
#[macro_export]
macro_rules! impl_has_vault {
    ($($t: ty), + $(,)?) => ($(
        impl $crate::instructions::reconcile::HasVault for $t {
            fn vault(&self) -> &Vault {
                self.vault.deref()
            }

            fn vault_mut(&mut self) -> &mut Vault {
                self.vault.deref_mut()
            }
        }
    )+)
}

pub fn handle<T: LendingMarket + HasVault>(ctx: Context<T>, withdraw_option: u64) -> Result<()> {
    // Check that reconciles are not halted
    (!ctx
        .accounts
        .vault()
        .flags()
        .contains(VaultFlags::HALT_RECONCILES))
    .ok_or::<Error>(ErrorCode::HaltedVault.into())?;

    

    Ok(())
}
