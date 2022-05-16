use std::{
    io::Write,
    ops::{Deref, DerefMut},
};

use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{Token, TokenAccount};
use solana_maths::Rate;
use spl_token_lending::state::Reserve;

use crate::{
    impl_has_vault,
    reconcile::LendingMarket,
    reserves::{Provider, ReserveAccessor},
    state::Vault
};
