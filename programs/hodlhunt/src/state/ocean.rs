use crate::constants::ocean;
use anchor_lang::prelude::*;

#[account]
pub struct Ocean {
    pub admin: Pubkey,
    pub total_fish_count: u64,
    pub total_shares: u64,
    pub balance_fishes: u64,
    pub vault_bump: u8,
    pub last_feeding_update: i64,
    pub next_fish_id: u64,
    pub vault: Pubkey,
    pub is_storm: bool,
    pub feeding_percentage: u16,
    pub storm_probability_bps: u16,
    pub last_cycle_mode: u8,
    pub cycle_start_time: i64,
    pub next_mode_change_time: i64,
}

impl Ocean {
    pub const INIT_SPACE: usize = 32 + 8 + 8 + 8 + 1 + 8 + 8 + 32 + 1 + 2 + 2 + 1 + 8 + 8;

    /// Returns `true` when the provided timestamp has reached the scheduled mode change time.
    pub fn should_change_mode(&self, current_time: i64) -> bool {
        current_time >= self.next_mode_change_time
    }

    /// Uses the stored storm probability to decide whether the next mode should be stormy.
    pub fn determine_next_mode(&self, random_seed: u64) -> bool {
        let storm_chance = ocean::INITIAL_STORM_PROBABILITY_BPS as u64;
        let random_roll = random_seed % 1000;
        let will_storm = random_roll < storm_chance;
        msg!(
            "Mode decision: random {} vs storm chance {}, result: {}",
            random_roll,
            storm_chance,
            if will_storm { "STORM" } else { "CALM" }
        );
        will_storm
    }

    /// Applies a mode transition, updates scheduling metadata, and emits an event that
    /// captures the change alongside the provided reason string.
    pub fn apply_mode_change(&mut self, new_mode: bool, current_time: i64, reason: &str) {
        let old_mode = self.is_storm;
        let old_feeding_percentage = self.feeding_percentage;
        self.is_storm = new_mode;
        self.feeding_percentage = if new_mode {
            ocean::STORM_FEEDING_BPS
        } else {
            ocean::CALM_FEEDING_BPS
        };
        self.last_cycle_mode = if new_mode { 1 } else { 0 };
        self.storm_probability_bps = ocean::INITIAL_STORM_PROBABILITY_BPS;
        self.cycle_start_time = Self::current_day_start(current_time);
        self.next_mode_change_time = Self::next_midnight(current_time);
        emit!(crate::OceanModeChanged {
            old_mode,
            new_mode,
            old_feeding_percentage,
            new_feeding_percentage: self.feeding_percentage,
            storm_probability_bps: self.storm_probability_bps,
            cycle_start_time: self.cycle_start_time,
            next_change_time: self.next_mode_change_time,
            reason: reason.to_string(),
            timestamp: current_time,
        });
        let feeding_percent = self.feeding_percentage as f64 / 100.0;
        msg!(
            "Ocean mode changed: {} -> {} (feeding: {:.2}%), next change at {}",
            if old_mode { "STORM" } else { "CALM" },
            if new_mode { "STORM" } else { "CALM" },
            feeding_percent,
            self.next_mode_change_time
        );
    }

    /// Computes the start timestamp of the day that contains `timestamp`.
    fn current_day_start(timestamp: i64) -> i64 {
        timestamp - timestamp.rem_euclid(ocean::DAY_DURATION)
    }

    /// Computes the timestamp of the next midnight following `timestamp`.
    fn next_midnight(timestamp: i64) -> i64 {
        let remainder = timestamp.rem_euclid(ocean::DAY_DURATION);
        if remainder == 0 {
            timestamp + ocean::DAY_DURATION
        } else {
            timestamp + (ocean::DAY_DURATION - remainder)
        }
    }
}
