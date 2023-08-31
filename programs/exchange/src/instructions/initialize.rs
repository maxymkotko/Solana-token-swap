use crate::constants::*;
use crate::pool::Pool;
use crate::Fee;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::Token;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init,seeds=[INITIALIZE_POOL_TAG,payer.key.as_ref()],bump, payer=payer,space=Pool::MAX_SIZE)]
    pub pool: Account<'info, Pool>,
    #[account(owner=pool.key())]
    pub token_a: Account<'info, TokenAccount>,
    #[account(owner=pool.key())]
    pub token_b: Account<'info, TokenAccount>,
    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,
    #[account(token::mint=pool_mint)]
    pub pool_token_reciept_account: Account<'info, TokenAccount>,
    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializePool<'info> {
    pub fn initialize(&mut self, fees: Fee) -> Result<()> {
        let pool = &mut self.pool;
        pool.fees = fees;
        pool.token_a = self.token_a.key();
        pool.token_b = self.token_b.key();
        pool.token_a_mint = self.token_a.mint;
        pool.token_b_mint = self.token_b.mint;

        // mint pool tokens to receipt account for initial input
        Ok(())
    }
}
