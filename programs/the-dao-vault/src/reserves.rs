#[cfg(test)]
use mockall::*;

use anchor_lang::prelude::*;
use port_anchor_adaptor::PortReserve;
use solana_maths::{Rate, TryMul};
use strum_macros::{EnumCount, EnumIter};

use crate::adapters::solend::SolendReserve;

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

#[cfg_attr(test, automock)]
pub trait ReserveAccessor {
    fn utilization_rate(&self) -> Result<Rate>;
    fn borrow_rate(&self) -> Result<Rate>;

    fn reserve_with_deposit(&self, allocation: u64) -> Result<Box<dyn ReserveAccessor>>;
}

#[cfg_attr(test, automock)]
pub trait ReturnCalculator {
    fn calculate_return(&self, allocation: u64) -> Result<Rate>;
}

impl<T> ReturnCalculator for T
where
    T: ReserveAccessor,
{
    fn calculate_return(&self, allocation: u64) -> Result<Rate> {
        let reserve = self.reserve_with_deposit(allocation)?;
        let res = reserve.utilization_rate()?.try_mul(reserve.borrow_rate()?);
        match res {
            Ok(val) => return Ok(val),
            Err(err) => return Err(err.into()),
        }
    }
}

#[derive(Clone)]
pub enum Reserves {
    Solend(SolendReserve),
    Port(PortReserve),
}

impl<'a> ReserveAccessor for Reserves {
    fn utilization_rate(&self) -> Result<Rate> {
        match self {
            Reserves::Solend(reserve) => reserve.utilization_rate(),
            Reserves::Port(reserve) => reserve.utilization_rate()
        }
    }
}
