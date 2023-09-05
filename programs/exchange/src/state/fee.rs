use anchor_lang::prelude::*;

#[account]
pub struct Fee {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,

    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,

    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denomiator: u64,
}
