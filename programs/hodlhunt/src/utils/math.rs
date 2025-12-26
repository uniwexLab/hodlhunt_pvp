use crate::constants::fees;
use crate::{Fish, Ocean};
use solana_safe_math::SafeMath;

/// Converts a share amount into its value in lamports, rounding to the nearest
/// whole lamport to minimize precision loss when distributing rewards.
pub fn share_to_value(ocean: &Ocean, share: u64) -> u64 {
    if ocean.total_shares == 0 {
        return 0;
    }
    // Rounded to nearest: (a*b + denom/2) / denom using SafeMath
    let a = share as u128;
    let b = ocean.balance_fishes as u128;
    let denom = ocean.total_shares as u128;
    let numerator = a
        .safe_mul(b)
        .unwrap_or(0)
        .safe_add(denom.safe_div(2).unwrap_or(0))
        .unwrap_or(0);
    numerator.safe_div(denom).unwrap_or(0) as u64
}

/// Computes the number of shares that correspond to a deposited value, maintaining
/// proportional ownership of the ocean while rounding to the nearest share.
pub fn new_share(ocean: &Ocean, value: u64) -> u64 {
    if ocean.total_shares == 0 {
        value
    } else {
        let a = value as u128;
        let b = ocean.total_shares as u128;
        let denom = ocean.balance_fishes.saturating_sub(value) as u128;
        if denom == 0 {
            return 0;
        }
        let numerator = a
            .safe_mul(b)
            .unwrap_or(0)
            .safe_add(denom.safe_div(2).unwrap_or(0))
            .unwrap_or(0);
        numerator.safe_div(denom).unwrap_or(0) as u64
    }
}

/// Returns the raw feeding requirement for the provided share amount, applying the
/// current ocean feeding percentage and ignoring hunt refunds or minimum thresholds.
pub fn base_feeding_requirement(ocean: &Ocean, share: u64) -> u64 {
    let feeding_percent = ocean.feeding_percentage as u64;
    let value = share_to_value(ocean, share);
    value.saturating_mul(feeding_percent) / 10_000
}

/// Calculates the minimum lamports a player must spend to feed a fish, accounting for
/// prior hunt rewards that offset feeding costs and enforcing the global minimum.
pub fn min_feeding_amount(ocean: &Ocean, fish: &Fish) -> u64 {
    base_feeding_requirement(ocean, fish.share)
        .saturating_sub(fish.received_from_hunt_value)
        .max(fees::MIN_FEED_LAMPORTS)
}
