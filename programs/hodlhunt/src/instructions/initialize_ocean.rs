use crate::constants::ocean;
use crate::instructions::contexts::initialize_ocean::*;
use anchor_lang::prelude::*;

/// Creates the primary ocean state, sets initial parameters, and schedules the first
/// daily mode change. Must run only once before any game activity begins.
pub fn handle(ctx: Context<InitializeOcean>) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let current_time = Clock::get()?.unix_timestamp;
    ocean.admin = ctx.accounts.declared_admin.key();
    ocean.total_fish_count = 0;
    ocean.total_shares = 0;
    ocean.balance_fishes = 0;
    ocean.vault_bump = ctx.bumps.vault;
    ocean.vault = ctx.accounts.vault.key();
    ocean.last_feeding_update = current_time;
    ocean.next_fish_id = 1;
    ocean.is_storm = false;
    ocean.feeding_percentage = ocean::CALM_FEEDING_BPS;
    ocean.storm_probability_bps = ocean::INITIAL_STORM_PROBABILITY_BPS;
    ocean.last_cycle_mode = 255;
    let day_start = current_time - current_time.rem_euclid(ocean::DAY_DURATION);
    ocean.cycle_start_time = day_start;
    let next_midnight = if current_time.rem_euclid(ocean::DAY_DURATION) == 0 {
        current_time + ocean::DAY_DURATION
    } else {
        day_start + ocean::DAY_DURATION
    };
    ocean.next_mode_change_time = next_midnight;
    Ok(())
}
