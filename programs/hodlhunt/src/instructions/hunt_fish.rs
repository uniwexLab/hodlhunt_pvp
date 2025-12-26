use crate::constants::fees;
use crate::errors::ErrorCode;
use crate::instructions::common::release_name_if_dead;
use crate::Fish;
use crate::{events::*, instructions::HuntFish, utils::*};
use anchor_lang::prelude::*;

/// Executes a hunt between two fish, enforcing cooldowns, mark exclusivity, size checks,
/// and distributing the prey share among hunter, pool and admin. Updates cooldowns and
/// ensures the hunter has resources to cover subsequent feeding requirements.
pub fn handle(ctx: Context<HuntFish>, expected_prey_share: u64) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let hunter = &mut ctx.accounts.hunter;
    let prey = &mut ctx.accounts.prey;
    let vault = &mut ctx.accounts.vault;
    let hunter_owner = &mut ctx.accounts.hunter_owner;
    let admin = &mut ctx.accounts.admin;
    let system_program = &ctx.accounts.system_program;

    let current_time = Clock::get()?.unix_timestamp;

    hunter.ensure_alive()?;
    prey.ensure_alive()?;
    hunter.ensure_owned_by(&hunter_owner.key())?;
    require!(hunter.id != prey.id, ErrorCode::InvalidPrey);
    require!(hunter.owner != prey.owner, ErrorCode::InvalidPrey);
    require!(hunter.share > prey.share, ErrorCode::PreyTooHeavy);

    require!(hunter.can_hunt(current_time), ErrorCode::HuntingOnCooldown);
    require!(prey.is_valid_prey(current_time), ErrorCode::InvalidPrey);

    check_hunting_mark_exclusivity(prey, hunter.id, current_time)?;

    let lower_bound = expected_prey_share.saturating_mul(95).saturating_div(100);
    let upper_bound = expected_prey_share.saturating_mul(105).saturating_div(100);
    require!(
        prey.share >= lower_bound && prey.share <= upper_bound,
        ErrorCode::SlippageExceeded
    );

    let bite_share = prey.share;

    let to_hunter = bite_share.saturating_mul(80).saturating_div(100);
    let to_pool = bite_share.saturating_mul(10).saturating_div(100);
    let to_admin_share = bite_share.saturating_mul(10).saturating_div(100);

    let to_pool_value = share_to_value(ocean, to_pool);
    let to_admin_value = share_to_value(ocean, to_admin_share);

    prey.share = 0;
    hunter.share = hunter.share.saturating_add(to_hunter);

    ocean.total_shares = ocean
        .total_shares
        .checked_sub(to_admin_share + to_pool)
        .ok_or(ErrorCode::MathOverflow)?;
    ocean.balance_fishes = ocean
        .balance_fishes
        .checked_sub(to_admin_value)
        .ok_or(ErrorCode::MathOverflow)?;

    if to_admin_value > 0 {
        transfer_to_admin(
            ocean,
            &ocean.key(),
            vault,
            admin,
            system_program,
            to_admin_value,
        )?;
    }

    let min_feeding_value = base_feeding_requirement(ocean, hunter.share)
        .max(fees::MIN_FEED_LAMPORTS);

    let received_from_hunt_value = share_to_value(ocean, to_hunter);

    hunter.last_hunt_at = current_time;
    hunter.can_hunt_after = current_time + Fish::POST_HUNT_COOLDOWN;
    if received_from_hunt_value >= min_feeding_value {
        hunter.last_fed_at = current_time;
        hunter.received_from_hunt_value = 0;
    } else {
        hunter.received_from_hunt_value = received_from_hunt_value;
    }

    hunter.total_hunts = hunter.total_hunts.saturating_add(1);
    hunter.total_hunt_income = hunter
        .total_hunt_income
        .saturating_add(received_from_hunt_value);

    
    release_name_if_dead(
        prey,
        &ctx.accounts.prey_name_registry,
        &prey.to_account_info(),
    )?;
    
    ocean.total_fish_count = ocean.total_fish_count.saturating_sub(1);

    emit!(FishHunted {
        hunter_id: hunter.id,
        prey_id: prey.id,
        hunter_owner: hunter.owner,
        prey_owner: prey.owner,
        bite_share,
        to_hunter,
        to_pool,
        to_admin: to_admin_share,
        enhanced: false,
        hunter_new_share: hunter.share,
        prey_new_share: prey.share,
        received_from_hunt_value: received_from_hunt_value,
        to_admin_value,
        to_pool_value,
        bite_percent: 100,
        bite_fee_percent: 0,
        bite_fee: 0,
    });

    Ok(())
}
