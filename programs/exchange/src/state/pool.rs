use anchor_lang::prelude::*;

use crate::Fee;
#[account]
pub struct Pool {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee_account: Pubkey,
    pub fees: Fee,
}

impl Pool {
    pub const MAX_SIZE: usize = 4 + 32 * 2;
}
