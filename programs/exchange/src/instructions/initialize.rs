use crate::constants::*;
use crate::pool::Pool;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init,seeds=[INITIALIZE_POOL_TAG,payer.key.as_ref()],bump, payer=payer,space=Pool::MAX_SIZE)]
    pub swap: Account<'info, Pool>,
    pub token_a: AccountInfo<'info>,
    pub token_b: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
