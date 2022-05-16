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
    state::Vault,
};

#[derive(Accounts)]
pub struct SolendAccounts<'info> {
    /// Vault state account
    /// Checks that the accounts passed in are correct
    #[account(mut, has_one = vault_authority, has_one = vault_reserve_token, has_one = vault_solend_lp_token, has_one = solend_reserve, )]
    pub vault: Box<Account<'info, Vault>>,

    /// Authority that the vault uses for lp token mints/burns ans transfers to/from downstream assets
    pub vault_authority: AccountInfo<'info>,

    /// Token account for the vault's reserve tokens
    #[account(mut)]
    pub vault_reserve_token: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub vault_solend_lp_token: Box<Account<'info, TokenAccount>>,

    #[account(executable, address = spl_token_lending::ID)]
    pub solend_program: AccountInfo<'info>,

    pub solend_market_authority: AccountInfo<'info>,

    pub solend_market: AccountInfo<'info>,

    #[account(mut)]
    pub solend_reserve: Box<Account<'info, SolendReserve>>,

    #[account(mut)]
    pub solend_lp_mint: AccountInfo<'info>,

    #[account(mut)]
    pub solend_reserve_token: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,

    pub token_program: Program<'info, Token>,
}

impl_has_vault!(SolendAccounts<'_>);

impl<'info> LendingMarket for SolendAccounts<'info> {
    fn deposit(&self, amount: u64) -> Result<()> {
        let context = CpiContext::new(self.solend_program.clone(), DepositReserveLiquidity {});

        Ok(())
    }
}

#[derive(Clone)]
pub struct SolendReserve(Reserve);

impl anchor_lang::AccountDeserialize for SolendReserve {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        SolendReserve::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        match <Reserve as solana_program::program_pack::Pack>::unpack(buf).map(SolendReserve) {
            Ok(val) => Ok(val),
            Err(err) => Err(err.into()),
        }
    }
}

impl anchor_lang::AccountSerialize for SolendReserve {
    fn try_serialize<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl anchor_lang::Owner for SolendReserve {
    fn owner() -> Pubkey {
        spl_token_lending::id()
    }
}

#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    // Lending program
    pub lending_program: AccountInfo<'info>,

    // Token account for asset to deposit into reserve
    pub source_liquidity: AccountInfo<'info>,

    // Token account for reserve collateral token
    pub destination_collateral_account: AccountInfo<'info>,

    // Reserve state account
    pub reserve: AccountInfo<'info>,

    // Token mint for reserve collateral token
    pub reserve_collateral_mint: AccountInfo<'info>,

    // Reserve liquidity supply SPL token account
    pub reserve_liquidity_supply: AccountInfo<'info>,

    // Lending market
    pub lending_market: AccountInfo<'info>,

    // Lending market Authority (PDA)
    pub lending_market_authority: AccountInfo<'info>,

    // Transfer auhtority for accounts 1 and 2
    pub transfer_authority: AccountInfo<'info>,

    // Clock
    pub clock: AccountInfo<'info>,

    // Token program ID
    pub token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    // Lending program
    pub lending_program: AccountInfo<'info>,

    // Source token account for reserve collateral token
    pub source_collateral: AccountInfo<'info>,

    // Destination liquidity token account
    pub destination_liquidity: AccountInfo<'info>,

    // Refreshed reserve account
    pub reserve: AccountInfo<'info>,

    // Reserve collateral mint account
    pub reserve_collateral_mint: AccountInfo<'info>,

    // Reserve liquidity supply SPL Token account.
    pub reserve_liquidity_supply: AccountInfo<'info>,

    // Lending market
    pub lending_market: AccountInfo<'info>,

    // Lending market account - PDA
    pub lending_market_authority: AccountInfo<'info>,

    // User transfer authority
    pub transfer_authority: AccountInfo<'info>,

    // Clock
    pub clock: AccountInfo<'info>,

    // Token program ID
    pub token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RefreshReserve<'info> {
    // Lending program
    pub lending_program: AccountInfo<'info>,

    // Reserve account
    pub reserve: AccountInfo<'info>,

    // Pyth reserve liquidity oracle
    // Must be the pyth price account specified in InitReserve
    pub pyth_reserve_liquidity_oracle: AccountInfo<'info>,

    // Switchboard Reserve liquidity oracle account
    // Must be the switchboard price account specified in InitReserve
    pub switchboard_reserve_liquidity_oracle: AccountInfo<'info>,

    // Clock
    pub clock: AccountInfo<'info>,
}
