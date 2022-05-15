use std::convert::TryFrom;

use anchor_lang::{
    prelude::ProgramError,
    solana_program::clock::{DEFAULT_HASHES_PER_SECOND, DEFAULT_TICKS_PER_SLOT, SECONDS_PER_DAY},
};
use spl_math::precise_number::PreciseNumber;

use crate::errors::ErrorCode;

pub const INITIAL_COLLATERAL_RATIO: u64 = 1;

pub fn calc_reserve_to_lp(
    reserve_token_amount: u64,
    lp_token_supply: u64,
    reserve_tokens_in_vault: u64,
) -> Option<u64> {
    match reserve_tokens_in_vault {
        // Assert that lp supply is 0
        0 => Some(INITIAL_COLLATERAL_RATIO.checked_mul(reserve_token_amount)?),
        _ => {
            let reserve_token_amount = PreciseNumber::new(reserve_token_amount as u128)?;
            let lp_token_supply = PreciseNumber::new(lp_token_supply as u128)?;
            let reserve_tokens_in_vault = PreciseNumber::new(reserve_tokens_in_vault as u128)?;

            let lp_tokens_to_mint = lp_token_supply
                .checked_mul(&reserve_token_amount)?
                .checked_div(&reserve_tokens_in_vault)?
                .floor()?
                .to_imprecise()?;

            u64::try_from(lp_tokens_to_mint).ok()
        }
    }
}
