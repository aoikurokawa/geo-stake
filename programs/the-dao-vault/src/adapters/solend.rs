use std::{
    io::Write,
    ops::{Deref, DerefMut},
};

use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{Token, TokenAccount};
use solana_maths::Rate;
use spl_token
