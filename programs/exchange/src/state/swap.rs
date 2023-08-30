use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Initialize {
    fee: u32,
    token_a: Pubkey,
    token_b: Pubkey,
}
