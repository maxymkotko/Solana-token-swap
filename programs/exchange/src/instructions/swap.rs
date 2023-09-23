use crate::constants::*;
use crate::errors::ExchangeError;
use crate::{curve::constant_product::*, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};
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

    #[account(owner=pool_authority.key())]
    pub source_account: Account<'info, TokenAccount>,

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
        let (
            new_pool_source_amount,
            new_pool_destination_amount,
            swapped_source_amount,
            swapped_destination_amount,
            owner_fee,
            trading_fee,
        ) = swap(
            source_amount as u128,
            self.pool_source_account.amount as u128,
            self.pool_destination_account.amount as u128,
            &self.pool.fees,
        )?;

        // transfer the swapped amounts
        let source_transfer_accounts = Transfer {
            authority: self.user.to_account_info(),
            to: self.pool_source_account.to_account_info(),
            from: self.source_account.to_account_info(),
        };
        let source_transfer_context = CpiContext::new(
            self.token_program.to_account_info(),
            source_transfer_accounts,
        );
        transfer(source_transfer_context, swapped_source_amount as u64)?;

        let destination_transfer_accounts = Transfer {
            authority: self.pool_authority.to_account_info(),
            to: self.destination_account.to_account_info(),
            from: self.pool_destination_account.to_account_info(),
        };

        let pool_key_ref = self.pool.key().as_ref().to_owned();
        let signer_seeds = &[INITIALIZE_POOL_TAG, &pool_key_ref, &[self.pool.bump]];
        let signer = &[&signer_seeds[..]];

        let destination_transfer_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            destination_transfer_accounts,
            signer,
        );
        transfer(
            destination_transfer_context,
            swapped_destination_amount as u64,
        )?;

        // mint the pool_tokens propotional to owner_fee to pool_fee_account
        let pool_tokens = calculate_withdraw_single_token_out(
            owner_fee,
            new_pool_source_amount,
            self.pool_mint.supply as u128,
        )?;
        let pool_mint_to_accounts = MintTo {
            authority: self.pool_authority.to_account_info(),
            mint: self.pool_mint.to_account_info(),
            to: self.pool_token_fee_account.to_account_info(),
        };
        let pool_mint_to_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            pool_mint_to_accounts,
            signer,
        );
        mint_to(pool_mint_to_context, pool_tokens as u64)?;

        Ok(())
    }
}
