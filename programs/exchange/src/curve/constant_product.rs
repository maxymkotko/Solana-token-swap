use crate::fee::*;
use anchor_lang::Result;
use spl_math::{checked_ceil_div::CheckedCeilDiv, precise_number::PreciseNumber};

// Constant product swap : (A+A') * (B-B') = invariant
pub fn swap(
    source_amount: u128,
    pool_source_amount: u128,
    pool_destination_amount: u128,
    fee: &Fee,
) -> Result<(u128, u128, u128, u128, u128, u128)> {
    // Calculate the fee
    let trading_fee = calculate_fee(
        source_amount,
        fee.trade_fee_numerator,
        fee.trade_fee_denominator,
    )
    .unwrap();

    let owner_fee = calculate_fee(
        source_amount,
        fee.owner_trade_fee_numerator,
        fee.owner_trade_fee_denominator,
    )
    .unwrap();

    let total_fee = trading_fee.checked_add(owner_fee).unwrap();
    let source_amount_after_fee = source_amount.checked_sub(total_fee).unwrap();

    // invariant = (A*B)
    let invariant = pool_source_amount
        .checked_mul(pool_destination_amount)
        .unwrap();
    // A + A'
    let total_source_amount = pool_source_amount
        .checked_add(source_amount_after_fee)
        .unwrap();
    // B - B' = invariant/(A+A');
    let (total_destination_amount, total_source_amount) =
        invariant.checked_ceil_div(total_source_amount).unwrap();

    // B' = B - invariant/(A+A')
    let swapped_destination_amount = pool_destination_amount
        .checked_sub(total_destination_amount)
        .unwrap();

    // A' = total_source - A
    let swapped_source_amount = total_source_amount
        .checked_sub(pool_source_amount)
        .unwrap()
        .checked_add(total_fee)
        .unwrap();

    let new_pool_source_amount = source_amount.checked_add(swapped_source_amount).unwrap();
    let new_pool_destination_amount = pool_destination_amount
        .checked_sub(swapped_destination_amount)
        .unwrap();

    Ok((
        new_pool_source_amount,
        new_pool_destination_amount,
        swapped_source_amount,
        swapped_destination_amount,
        owner_fee,
        trading_fee,
    ))
}

pub fn calculate_deposit_single_token_out(
    source_amount: u128,
    pool_source_amount: u128,
    pool_supply: u128,
) -> Result<u128> {
    let source_amount = PreciseNumber::new(source_amount).unwrap();
    let pool_source_amount = PreciseNumber::new(pool_source_amount).unwrap();
    let pool_supply = PreciseNumber::new(pool_supply).unwrap();
    let one = PreciseNumber::new(1).unwrap();
    let ratio_deposited = one
        .checked_add(&source_amount.checked_div(&pool_source_amount).unwrap())
        .unwrap();
    let ratio = ratio_deposited.sqrt().unwrap().checked_sub(&one).unwrap();
    let result_amount = pool_supply.checked_mul(&ratio).unwrap();
    Ok(result_amount.to_imprecise().unwrap())
}

pub fn calculate_withdraw_single_token_out(
    source_amount: u128,
    new_pool_source_amount: u128,
    result_supply: u128,
) -> Result<u128> {
    let source_amount = PreciseNumber::new(source_amount).unwrap();
    let source_amount_supply = PreciseNumber::new(new_pool_source_amount).unwrap();
    let result_supply = PreciseNumber::new(result_supply).unwrap();

    let ratio_redeemed = source_amount.checked_div(&source_amount_supply).unwrap();
    let one = PreciseNumber::new(1).unwrap();
    let ratio_redeemed = one.checked_sub(&ratio_redeemed).unwrap();
    let ratio = one.checked_sub(&ratio_redeemed.sqrt().unwrap()).unwrap();
    let result_amount = result_supply.checked_mul(&ratio).unwrap();
    Ok(result_amount.to_imprecise().unwrap())
}

fn calculate_fee(source_amount: u128, fee_numerator: u64, fee_denominator: u64) -> Option<u128> {
    let fee: u128 = source_amount
        .checked_mul(fee_numerator as u128)?
        .checked_div(fee_denominator as u128)?;

    if fee == 0 {
        Some(1)
    } else {
        Some(fee)
    }
}
