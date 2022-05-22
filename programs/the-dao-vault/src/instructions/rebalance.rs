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
    reserves::{Provider, Reserves},
    state::*,
};
