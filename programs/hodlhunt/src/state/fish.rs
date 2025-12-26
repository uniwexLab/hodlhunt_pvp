use anchor_lang::prelude::*;

use crate::constants::{marks, ocean};

use crate::errors::ErrorCode;

#[account]
#[derive(Default)]
pub struct Fish {
    pub id: u64,
    pub owner: Pubkey,
    pub share: u64,
    pub name: String,
    pub created_at: i64,
    pub last_fed_at: i64,
    pub last_hunt_at: i64,
    pub can_hunt_after: i64,
    pub is_protected: bool,
    pub protection_ends_at: i64,
    pub total_hunts: u64,
    pub total_hunt_income: u64,
    pub received_from_hunt_value: u64,
    pub hunting_marks_placed: u8,
    pub last_mark_reset: i64,
    pub marked_by_hunter_id: u64,
    pub mark_placed_at: i64,
    pub mark_expires_at: i64,
    pub mark_cost: u64,
}

impl Fish {
    pub const INIT_SPACE: usize = 190;

    pub const PROTECTION_PERIOD: i64 = 7 * ocean::DAY_DURATION;
    pub const CREATION_HUNTING_COOLDOWN: i64 = 2 * ocean::DAY_DURATION;
    pub const POST_HUNT_COOLDOWN: i64 = 2 * ocean::DAY_DURATION;
    pub const PREY_COOLDOWN: i64 = 7 * ocean::DAY_DURATION;
    pub const FEEDING_COOLDOWN: i64 = 2 * ocean::DAY_DURATION;
    pub const MARK_EXCLUSIVITY_PERIOD: i64 = marks::EXCLUSIVITY_SECONDS;

    /// Returns true when the fish can initiate a hunt at the provided timestamp.
    pub fn can_hunt(&self, current_time: i64) -> bool {
        current_time >= self.can_hunt_after && self.share > 0
    }

    /// Checks if the active hunting mark has expired relative to the provided time.
    pub fn is_mark_expired(&self, current_time: i64) -> bool {
        self.marked_by_hunter_id > 0 && current_time > self.mark_expires_at
    }

    /// Clears mark metadata when the exclusivity window has expired.
    pub fn clear_expired_mark(&mut self, current_time: i64) {
        if self.is_mark_expired(current_time) {
            self.marked_by_hunter_id = 0;
            self.mark_placed_at = 0;
            self.mark_expires_at = 0;
            self.mark_cost = 0;
        }
    }

    /// Verifies that the fish is a valid prey candidate at the given timestamp.
    pub fn is_valid_prey(&self, current_time: i64) -> bool {
        if self.share == 0 {
            return false;
        }
        if self.is_protected && current_time < self.protection_ends_at {
            return false;
        }
        let time_since_feeding = current_time - self.last_fed_at;
        if time_since_feeding < Self::PREY_COOLDOWN {
            return false;
        }
        true
    }

    /// Ensures the fish has non-zero share and is considered alive.
    pub fn ensure_alive(&self) -> Result<()> {
        require!(self.share > 0, ErrorCode::FishAlreadyDead);
        Ok(())
    }

    /// Ensures the fish has zero share and is considered dead.
    pub fn ensure_dead(&self) -> Result<()> {
        require!(self.share == 0, ErrorCode::FishAlreadyDead);
        Ok(())
    }

    /// Validates that the fish is owned by the provided public key.
    pub fn ensure_owned_by(&self, owner: &Pubkey) -> Result<()> {
        require!(self.owner == *owner, ErrorCode::NotFishOwner);
        Ok(())
    }
}
