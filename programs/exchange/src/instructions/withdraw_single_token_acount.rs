use crate::errors::ExchangeError;
use crate::{constants::*, Fee};
use crate::{curve::constant_product::*, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer};
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;

#[derive(Accounts)]
pub struct WithdrawSingleToken<'info> {
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
    pub pool_token_user_account: Account<'info, TokenAccount>,

    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,

    // mint of token to be withdrawn
    pub source_mint: Account<'info, Mint>,

    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawSingleToken<'info> {
    pub fn withdraw_single_token_out(&mut self, source_amount: u64, fees: Fee) -> Result<()> {
        if !cmp_pubkeys(&self.pool_source_account.mint, &self.source_account.mint) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if !cmp_pubkeys(&self.pool_destination_account.mint, &self.pool.pool_mint) {
            return Err(ExchangeError::InvalidMint.into());
        }
        if self.pool_token_user_account.amount < source_amount {
            return Err(ExchangeError::NotEnoughFunds.into());
        }
        let burn_pool_token_amount = calculate_withdraw_single_token_out(
            source_amount as u128,
            self.pool_source_account.amount as u128,
            self.pool_mint.supply as u128,
        )?;

        // Todo: withdraw fee is 0 if withdrawal is from pool_fee_account
        let withdraw_fee = calculate_fee(
            source_amount as u128,
            fees.owner_withdraw_fee_numerator,
            fees.owner_trade_fee_denominator,
        )
        .unwrap();

        // transfer the withdraw fee
        let withdraw_fee_transfer_accounts = Transfer {
            to: self.pool_token_fee_account.to_account_info(),
            from: self.pool_source_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let withdraw_fee_transfer_context = CpiContext::new(
            self.token_program.to_account_info(),
            withdraw_fee_transfer_accounts,
        );
        transfer(withdraw_fee_transfer_context, withdraw_fee as u64)?;

        let pool_key_ref = self.pool.key().as_ref().to_owned();
        let signer_seeds = &[INITIALIZE_POOL_TAG, &pool_key_ref, &[self.pool.bump]];
        let signer = &[&signer_seeds[..]];

        // burn the pool tokens
        let burn_user_pool_tokens_account = Burn {
            mint: self.pool_mint.to_account_info(),
            from: self.pool_token_user_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let burn_pool_tokens_context = CpiContext::new(
            self.token_program.to_account_info(),
            burn_user_pool_tokens_account,
        );
        burn(burn_pool_tokens_context, burn_pool_token_amount as u64)?;

        // transfer the withdrawal source amount
        let source_amount_transfer_accounts = Transfer {
            to: self.source_account.to_account_info(),
            from: self.pool_source_account.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };

        let source_amount_transfer_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            source_amount_transfer_accounts,
            signer,
        );
        transfer(source_amount_transfer_context, source_amount as u64)?;

        Ok(())
    }
}
