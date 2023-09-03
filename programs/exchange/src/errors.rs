use anchor_lang::prelude::*;

#[error_code]
pub enum ExchangeError {
    #[msg("Pool mint supply should be zero")]
    PoolMintSupplyNotZero,

    #[msg("Authority is Invalid")]
    InvalidAuthority,
}
