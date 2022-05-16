use anchor_lang::prelude::*;
use std::cmp::Ordering;

use strum::IntoEnumIterator;
#[cfg(test)]
use type_layout::TypeLayout;

use jet_proto_proc_macros::assert_size;

use crate::{errors::ErrorCode, impl_provider_index};

// use crate::
#[assert_size(768)]
#[account]
#[repr(C, align(8))]
#[derive(Debug)]
#[cfg_attr(test, derive(TypeLayout))]
pub struct Vault {
    pub version: [u8; 3],

    pub owner: Pubkey,

    pub vault_authority: Pubkey,

    pub authority_seed: Pubkey,

    pub authority_bump: [u8; 1],

    pub solend_reserve: Pubkey,

    pub port_reserve: Pubkey,

    pub jet_reserve: Pubkey,

    pub vault_reserve_token: Pubkey,

    pub vault_solend_lp_token: Pubkey,

    pub vault_port_lp_token: Pubkey,

    pub vault_jet_lp_token: Pubkey,

    pub lp_token_mint: Pubkey,

    pub reserve_token_mint: Pubkey,

    pub fee_receiver: Pubkey,

    pub referral_fee_receiver: Pubkey,

    bitflags: u32,

    pub value: SlotTrackecValue,

    pub target_allocations: Allocations,

    pub config: VaultConfig,

    pub actual_allocations: Allocations,

    // 8 * 23 = 184
    /// Reserved spacce for future upgrades
    _reserved: [u64; 14],
}

impl Vault {
    pub fn flags(&self) -> VaultFlags {
        VaultFlags::from_bits(self.bitflags)
            .unwrap_or_else(|| panic!("{:?} does not resolve to build VaultFlags", self.bitflags))
    }

    pub fn set_flags(&mut self, bits: u32) -> Result<()> {
        VaultFlags::from_bits(bits).ok_or_else(|| return ErrorCode::InvalidVaultFlags)?;
        self.bitflags = bits;
        Ok(())
    }

    pub fn calculate_fees(&self, new_vault_value: u64, slot: u64) -> Result<u64> {
        let vault_value_diff = new_vault_value.saturating_sub(self.value.value);
        let slots_elapsed = self.value.last_update.slots_elapsed(slot)?;

        Ok(0)
    }
}

#[assert_size(aligns, 32)]
#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
#[cfg_attr(test, derive(TypeLayout))]
pub struct VaultConfig {
    pub deposit_cap: u64,
    pub fee_carry_bps: u32,
    pub fee_mgmt_bps: u32,
    pub referral_fee_pct: u8,
    pub allocation_cap_pct: u8,
    pub rebalance_mode: RebalanceMode,
    pub strategy_type: StrategyType,
    _padding: [u32; 3],
}

#[repr(u8)]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub enum RebalanceMode {
    Calculator,
    ProofChecker,
}

#[repr(u8)]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub enum StrategyType {
    MaxYield,
    EqualAllocation,
}

bitflags::bitflags! {
    pub struct VaultFlags: u32 {
        const HALT_RECONCILES = 1 << 0;
        const HALT_REFRESHED = 1 << 1;
        const HALT_DEPOSITS_WITHDRAWS = 1 << 2;
        const HALT_ALL = Self::HALT_RECONCILES.bits | Self::HALT_REFRESHED.bits | Self::HALT_DEPOSITS_WITHDRAWS.bits;
    }
}

#[assert_size(aligns, 72)]
#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct Allocations {
    pub solend: SlotTrackecValue,
    pub port: SlotTrackecValue,
    pub jet: SlotTrackecValue,
}

impl_provider_index!(Allocations, SlotTrackecValue);

#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct SlotTrackecValue {
    pub value: u64,
    pub last_update: LastUpdate,
}

// Number of slots to consider stale after
pub const STALE_AFTER_SLOTS_ELAPSED: u64 = 2;

#[assert_size(aligns, 16)]
#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct LastUpdate {
    pub slot: u64,
    pub stale: bool,
    _padding: [u8; 7],
}

impl LastUpdate {
    /// Create new last update
    pub fn new(slot: u64) -> Self {
        Self {
            slot,
            stale: true,
            _padding: [0_u8; 7],
        }
    }

    /// Return slots elapsed since given slot
    pub fn slots_elapsed(&self, slot: u64) -> Result<u64> {
        slot.checked_sub(self.slot)
            .ok_or_else(|| ErrorCode::MathError.into())
    }

    /// Set last update slot
    pub fn update_slot(&mut self, slot: u64) {
        self.slot = slot;
        self.stale = false;
    }

    /// Set stale to true
    pub fn mark_stale(&mut self) {
        self.stale = true;
    }

    /// Check if marked stale or last update slot is too long ago
    pub fn is_stale(&self, slot: u64) -> Result<bool> {
        #[cfg(feature = "debug")]
        {
            msg!("Last updated slot: {}", self.slot);
            msg!("Current slot: {}", slot);
        }

        Ok(self.stale || self.slots_elapsed(slot)? >= STALE_AFTER_SLOTS_ELAPSED)
    }
}

impl PartialEq for LastUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.slot == other.slot
    }
}

impl PartialOrd for LastUpdate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.slot.partial_cmp(&other.slot)
    }
}
