use anchor_lang::prelude::*;
mod constants;
mod curve;
mod errors;
mod instructions;
mod state;
mod utils;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod exchange {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, fees: Fee) -> Result<()> {
        ctx.accounts.initialize(fees)
    }

    pub fn swap(ctx: Context<Swap>, source_amount: u64) -> Result<()> {
        ctx.accounts.process_swap(source_amount)
    }

    pub fn deposit_all_tokens_in(
        ctx: Context<DepositAllTokens>,
        pool_tokens: u64,
        max_token_a: u64,
        max_token_b: u64,
    ) -> Result<()> {
        ctx.accounts
            .deposit_all_tokens_in(pool_tokens, max_token_a, max_token_b)
    }

    pub fn deposit_single_token(
        ctx: Context<DepositSingleToken>,
        source_amount: u64,
    ) -> Result<()> {
        ctx.accounts.deposit_single_token_in(source_amount)
    }

    pub fn withdraw_single_token_out(
        ctx: Context<WithdrawSingleToken>,
        source_amount: u64,
        fees: Fee,
    ) -> Result<()> {
        ctx.accounts.withdraw_single_token_out(source_amount, fees)
    }
}
