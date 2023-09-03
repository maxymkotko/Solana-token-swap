use anchor_lang::Result;
use spl_math::checked_ceil_div::CheckedCeilDiv;

// Constant product swap : (A+A') * (B-B') = invariant
pub fn swap(
    source_amount: u128,
    pool_source_amount: u128,
    pool_destination_amount: u128,
) -> Result<(u128, u128)> {
    //Todo: cut the fee from the source amount
    let source_amount_after_fee = source_amount;
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
