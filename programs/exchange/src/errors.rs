use anchor_lang::prelude::*;

#[error_code]
pub enum ExchangeError {
    #[msg("Pool mint supply should be zero")]
    PoolMintSupplyNotZero,

    #[msg("Authority is Invalid")]
    InvalidAuthority,

    #[msg("Mint in Invalid")]
    InvalidMint,

    #[msg("Not Enough Funds")]
    NotEnoughFunds,

    #[msg("Fail to convert the amount to specified type")]
    ConversionFailure,

    #[msg("Exceeded the slippage on trade")]
    SlippageExceeded,
}
