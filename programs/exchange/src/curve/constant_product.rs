use crate::fee::*;
use anchor_lang::Result;
use spl_math::checked_ceil_div::CheckedCeilDiv;

// Constant product swap : (A+A') * (B-B') = invariant
pub fn swap(
    source_amount: u128,
    pool_source_amount: u128,
    pool_destination_amount: u128,
    fee: &Fee,
) -> Result<(u128, u128)> {
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
    let swapped_source_amount = total_source_amount.checked_sub(pool_source_amount).unwrap();

    Ok((swapped_source_amount, swapped_destination_amount))
}

fn calculate_fee(source_amount: u128, fee_numerator: u64, fee_denominator: u64) -> Option<u128> {
    let fee: u128 = source_amount
        .checked_mul(fee_numerator as u128)?
        .checked_div(fee_numerator as u128)?;

    if fee == 0 {
        Some(1)
    } else {
        Some(fee)
    }
}
