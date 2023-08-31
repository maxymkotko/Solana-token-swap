use anchor_lang::prelude::*;
#[account]
pub struct Pool {
    fee: u32,
    token_a: Pubkey,
    token_b: Pubkey,
}

impl Pool {
    pub const MAX_SIZE: usize = 4 + 32 * 2;
}
