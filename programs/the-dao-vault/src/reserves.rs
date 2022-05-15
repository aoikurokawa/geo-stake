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

#[macro_export]
macro_rules! impl_provider_index {
    ($t: ty, $o: ty) => {
        impl core::ops::Index<Provider> for $t {
            type Output = $o;

            fn index(&self, provider: Provider) -> &Self::Output {
                match provider {
                    Provider::Solend => &mut self.solend,
                    Provider::Port => &self.port,
                    Provider::Jet => &self.jet,
                }
            }
        }

        impl core::ops::IndexMut<Provider> for $t {
            fn index_mut(&mut self, provider: Provider) -> &mut Self::Output {
                match provider {
                    Provider::Solend => &mut self.solend,
                    Provider::Port => &mut self.port,
                    Provider::Jet => &mut self.jet,
                }
            }
        }
    };
}
