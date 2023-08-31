use anchor_lang::prelude::*;

#[account]
pub struct Fee {
    trade_fee_numerator: u64,
    trade_fee_denominator: u64,

    owner_trade_fee_numerator: u64,
    owner_trade_fee_denominator: u64,

    owner_withdraw_fee_numerator: u64,
    owner_withdraw_fee_denomiator: u64,
}
