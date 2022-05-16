use anchor_lang::prelude::*;

pub mod errors;
pub mod math;
pub mod reserves;
pub mod state;
pub mod adapters;
pub mod instructions;

use adapters::*;
use instructions::*;

declare_id!("CYrQ5HhWsznvqX4dNMY8RuBxJhZQNjV6kZYynG22dqjQ");

#[program]
pub mod the_dao_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
