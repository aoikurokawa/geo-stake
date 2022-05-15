use anchor_lang::prelude::*;

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
