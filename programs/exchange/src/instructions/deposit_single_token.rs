use crate::constants::*;
use crate::errors::ExchangeError;
use crate::{curve::constant_product::*, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;

#[derive(Accounts)]
pub struct DepositSingleToken<'info> {
    #[account(seeds=[INITIALIZE_POOL_TAG,pool.key().as_ref()],bump)]
    pub pool_authority: AccountInfo<'info>,
    pub pool: Account<'info, Pool>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub pool_source_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub pool_destination_account: Account<'info, TokenAccount>,

    #[account(token::mint=source_mint,owner=user.key())]
    pub source_account: Account<'info, TokenAccount>,

    #[account(token::mint=pool_mint)]
    pub pool_token_recepient_account: Account<'info, TokenAccount>,

    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,

    // Token A mint
    pub source_mint: Account<'info, Mint>,

    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> DepositSingleToken<'info> {
    pub fn deposit_single_token_in(&mut self, source_amount: u64) -> Result<()> {
        if !cmp_pubkeys(&self.pool_source_account.mint, &self.source_account.mint) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if !cmp_pubkeys(&self.pool_destination_account.mint, &self.pool.pool_mint) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if self.source_account.amount < source_amount {
            return Err(ExchangeError::NotEnoughFunds.into());
        }
        let user_source_pool_tokens = calculate_deposit_single_token_out(
            source_amount as u128,
            self.pool_source_account.amount as u128,
            self.pool_mint.supply as u128,
        )?;

        // transfer the source amount
        let source_amount_transfer_accounts = Transfer {
            to: self.pool_source_account.to_account_info(),
            from: self.source_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let source_amount_transfer_context = CpiContext::new(
            self.token_program.to_account_info(),
            source_amount_transfer_accounts,
        );
        transfer(source_amount_transfer_context, source_amount as u64)?;

        let pool_key_ref = self.pool.key().as_ref().to_owned();
        let signer_seeds = &[INITIALIZE_POOL_TAG, &pool_key_ref, &[self.pool.bump]];
        let signer = &[&signer_seeds[..]];

        // mint deposited source amount propotional pool tokens
        let mint_pool_tokens_account = MintTo {
            to: self.source_account.to_account_info(),
            mint: self.pool_mint.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };
        let mint_pool_tokens_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_pool_tokens_account,
            signer,
        );
        mint_to(mint_pool_tokens_context, user_source_pool_tokens as u64)?;

        Ok(())
    }
}
