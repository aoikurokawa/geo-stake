use std::cmp::Ordering;

use strum::IntoEnumIterator;
#[cfg(test)]
use type_layout::TypeLayout;

use anchor_lang::prelude::*;
use jet_proto_proc_macros::assert_size;

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

#[assert_size(aligns, 72)]
#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct Allocations {
    pub solend: SlotTrackecValue,
    pub port: SlotTrackecValue,
    pub jet: SlotTrackecValue,
}

#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct SlotTrackecValue {
    pub value: u64,
    pub last_update: LastUpdate,
}

#[assert_size(aligns, 16)]
#[repr(C, align(8))]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default)]
pub struct LastUpdate {
    pub slot: u64,
    pub stale: bool,
    _padding: [u8; 7],
}
