use crate::constants::*;
use crate::errors::*;
use crate::pool::Pool;
use crate::utils::*;
use crate::Fee;

use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::mint_to;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init,seeds=[INITIALIZE_POOL_TAG,pool.key().as_ref()],bump, payer=payer,space=Pool::MAX_SIZE)]
    pub pool_authority: AccountInfo<'info>,
    pub pool: Account<'info, Pool>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub token_a: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub token_b: Account<'info, TokenAccount>,
    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,
    /// pool token reciept as per the tokenA|B input
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
        if self.pool_mint.mint_authority.is_none()
            || self.pool_mint.mint_authority.unwrap() != self.pool_authority.key()
        {
            return Err(ExchangeError::InvalidAuthority.into());
        }
        if self.pool_mint.supply != 0 {
            return Err(ExchangeError::PoolMintSupplyNotZero.into());
        }
        if self.pool_mint.freeze_authority.is_some() {
            return Err(ExchangeError::InvalidAuthority.into());
        }
        // assert close authority
        // validate fees
        let initial_supply: u64 = Pool::INITIAL_POOL_TOKEN_SUPPLY;
        let bump = get_bump(&[INITIALIZE_POOL_TAG, self.pool.key().as_ref()], &crate::ID);

        let pool_key = self.pool.key().clone();
        let signer_seeds = &[INITIALIZE_POOL_TAG, pool_key.as_ref(), &[bump]];
        let signer = &[&signer_seeds[..]];

        let cpi_accounts = MintTo {
            mint: self.pool_mint.to_account_info(),
            to: self.pool_token_reciept_account.to_account_info(),
            authority: self.pool_authority.clone(),
        };

        let cpi_context =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);
        mint_to(cpi_context, initial_supply)?;

        Ok(())
    }
}
