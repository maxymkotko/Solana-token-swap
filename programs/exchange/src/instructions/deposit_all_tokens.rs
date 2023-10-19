use crate::constants::*;
use crate::errors::ExchangeError;
use crate::{curve::constant_product::*, Pool};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};
use anchor_spl::token_interface::spl_token_2022::cmp_pubkeys;

#[derive(Accounts)]
pub struct DepositAllTokens<'info> {
    #[account(seeds=[INITIALIZE_POOL_TAG,pool.key().as_ref()],bump)]
    pub pool_authority: AccountInfo<'info>,
    pub pool: Account<'info, Pool>,
    /// Non-zero token A account
    #[account(owner=pool_authority.key())]
    pub pool_token_a_account: Account<'info, TokenAccount>,
    /// Non-zero token B account
    #[account(owner=pool_authority.key())]
    pub pool_token_b_account: Account<'info, TokenAccount>,

    #[account(token::mint=token_a_mint,owner=user.key())]
    pub token_a_account: Account<'info, TokenAccount>,

    #[account(token::mint=token_b_mint,owner=user.key())]
    pub token_b_account: Account<'info, TokenAccount>,

    #[account(token::mint=pool_mint)]
    pub pool_token_recepient_account: Account<'info, TokenAccount>,

    #[account(owner=pool.key())]
    pub pool_mint: Account<'info, Mint>,

    pub token_a_mint: Account<'info, Mint>,

    pub token_b_mint: Account<'info, Mint>,

    #[account(token::mint=pool_mint)]
    pub pool_token_fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> DepositAllTokens<'info> {
    pub fn deposit_all_tokens_in(
        &mut self,
        pool_tokens: u64,
        max_token_a: u64,
        max_token_b: u64,
    ) -> Result<()> {
        let pool = self.pool.clone();
        if !cmp_pubkeys(&pool.token_a_mint, &self.token_a_mint.key())
            || !cmp_pubkeys(&pool.token_b_mint, &self.token_b_mint.key())
        {
            return Err(ExchangeError::InvalidMint.into());
        }

        if !cmp_pubkeys(&pool.pool_mint, &self.pool_mint.key()) {
            return Err(ExchangeError::InvalidMint.into());
        }

        let (token_a_amount, token_b_amount) = convert_pool_tokens_to_trade_tokens(
            pool_tokens as u128,
            self.pool_mint.supply as u128,
            self.pool_token_a_account.amount as u128,
            self.pool_token_b_account.amount as u128,
        )
        .unwrap();

        if token_a_amount as u64 > max_token_a || token_b_amount as u64 > max_token_b {
            return Err(ExchangeError::SlippageExceeded.into());
        }

        let transfer_token_a_accounts = Transfer {
            from: self.token_a_account.to_account_info(),
            to: self.pool_token_a_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let transfer_token_a_context = CpiContext::new(
            self.token_program.to_account_info(),
            transfer_token_a_accounts,
        );
        transfer(transfer_token_a_context, token_a_amount as u64)?;

        let transfer_token_b_accounts = Transfer {
            from: self.token_b_account.to_account_info(),
            to: self.pool_token_b_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let transfer_token_b_context = CpiContext::new(
            self.token_program.to_account_info(),
            transfer_token_b_accounts,
        );
        transfer(transfer_token_b_context, token_b_amount as u64)?;

        let mint_to_accounts = MintTo {
            to: self.pool_token_recepient_account.to_account_info(),
            mint: self.pool_mint.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };
        let pool_key_ref = self.pool.key().as_ref().to_owned();
        let signer_seeds = &[INITIALIZE_POOL_TAG, &pool_key_ref, &[self.pool.bump]];
        let signer = &[&signer_seeds[..]];
        let mint_to_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_to_accounts,
            signer,
        );
        mint_to(mint_to_context, pool_tokens)?;

        Ok(())
    }
}
