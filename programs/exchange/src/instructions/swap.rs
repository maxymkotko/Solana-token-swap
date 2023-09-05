use crate::constants::*;
use crate::errors::ExchangeError;
use crate::{curve::constant_product::*, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(seeds=[INITIALIZE_POOL_TAG,pool.key().as_ref()],bump)]
    pub pool_authority: AccountInfo<'info>,
    pub pool: Account<'info, Pool>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub pool_source_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub pool_destination_account: Account<'info, TokenAccount>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub source_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub destination_account: Account<'info, TokenAccount>,
    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,
    // Token A mint
    pub source_mint: Account<'info, Mint>,
    // Token B mint
    pub destination_mint: Account<'info, Mint>,
    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Swap<'info> {
    pub fn process_swap(&mut self, source_amount: u64) -> Result<()> {
        if !cmp_pubkeys(&self.pool_source_account.mint, &self.source_account.mint) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if !cmp_pubkeys(
            &self.pool_destination_account.mint,
            &self.pool_destination_account.mint,
        ) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if self.user.lamports() < source_amount {
            return Err(ExchangeError::NotEnoughFunds.into());
        }
        let (swapped_source_amount, swapped_destination_amount) = swap(
            source_amount as u128,
            self.pool_source_account.amount as u128,
            self.pool_destination_account.amount as u128,
            &self.pool.fees,
        )?;
        // transfer the swapped amounts
        Ok(())
    }
}
