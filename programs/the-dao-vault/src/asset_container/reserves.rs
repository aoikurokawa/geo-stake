use core::{convert::TryFrom, ops::Index};
use std::cmp::Ordering;

use itertools::Itertools;
use solana_maths::{Rate, TryAdd, TryDiv, TryMul, TrySub};

use anchor_lang::prelude::*;

use crate::{
    errors::ErrorCode,
    reserves::{Provider, Reserves, ReturnCalculator},
    state::StrategyType,
};

use super::AssetContainer;

pub fn compare(lhs: &impl ReturnCalculator, rhs: &impl ReturnCalculator) -> Result<Ordering> {
    Ok(lhs.calculate_return(0)?.cmp(&rhs.calculate_return(0)?))
}

impl AssetContainer<Reserves> {
    fn calculate_weights_max_yield(&self, allocation_cap_pct: u8) -> Result<AssetContainer<Rate>> {
        self.into_iter()
            .sorted_unstable_by(|(_, alloc_y), (_, alloc_x)| {
                compare(*alloc_x, *alloc_y).expect("Could not successfully compare allocations")
            })
            .try_fold(
                (AssetContainer::<Rate>::default(), Rate::one()),
                |(mut strategy_weights, remaining_weight), (provider, _)| {
                    let target_weight =
                        remaining_weight.min(Rate::from_percent(allocation_cap_pct));
                    strategy_weights[provider] = target_weight;
                    match remaining_weight.try_sub(target_weight) {
                        Ok(r) => Ok((strategy_weights, r)),
                        Err(e) => Err(e.into()),
                    }
                },
            )
            .map(|(r, _)| r)
    }

    fn calculate_weight_equal(&self) -> Result<AssetContainer<Rate>> {
        u8::try_from(self.len())
            .map_err(|_| ErrorCode::StrategyError.into())
            .and_then(
                |num_assets| match Rate::from_percent(num_assets).try_mul(100) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(ErrorCode::MathError.into()),
                },
            )
            .and_then(|r| match Rate::one().try_div(r) {
                Ok(v) => Ok(v),
                Err(_) => Err(ErrorCode::MathError.into()),
            })
            .map(|equal_allocation| self.apply(|_, _| equal_allocation))
    }

    pub fn calculate_weights(
        &self,
        strategy_type: StrategyType,
        allocation_cap_pct: u8,
    ) -> Result<AssetContainer<Rate>> {
        match strategy_type {
            StrategyType::MaxYield => self.calculate_weights_max_yield(allocation_cap_pct),
            StrategyType::EqualAllocation => self.calculate_weight_equal(),
        }
    }

    pub fn get_apr(
        &self,
        weights: &dyn Index<Provider, Output = Rate>,
        allocations: &dyn Index<Provider, Output = u64>,
    ) -> Result<Rate> {
        let res = self
            .into_iter()
            .map(|(p, r)| {
                r.calculate_return(allocations[p])
                    .and_then(|r| match weights[p].try_mul(r) {
                        Ok(v) => Ok(v),
                        Err(err) => Err(err.into()),
                    })
            })
            .try_fold(Rate::zero(), |acc, r| acc.try_add(r?));

        match res {
            Ok(r) => Ok(r),
            Err(err) => Err(err.into()),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::reserves::MockReturnCalculator;
//     use solana_maths::Rate;

//     #[test]
//     fn test_compare() {
//         let mut mock_rc1 = MockReturnCalculator::new();
//         mock_rc1
//             .expect_calculate_return()
//             .return_const(Ok(Rate::from_percent(10)));

//         let mut mock_rc2 = MockReturnCalculator::new();
//         mock_rc1
//             .expect_calculate_return()
//             .return_const(Ok(Rate::from_percent(20)));

//         assert_eq!(compare(&mock_rc1, &mock_rc2), Ok(Ordering::Less));
//     }
// }
