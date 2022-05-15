#[cfg(test)]
use mockall::*;

use anchor_lang::prelude::*;
use solana_maths::{Rate, TryMul};
use strum_macros::{EnumCount, EnumIter};

// use crate::adap

#[derive(
    Clone,
    Copy,
    Debug,
    EnumIter,
    EnumCount,
    PartialEq,
    Ord,
    Hash,
    Eq,
    PartialOrd,
    AnchorSerialize,
    AnchorDeserialize,
)]
pub enum Provider {
    Solend = 0,
    Port,
    Jet,
}
