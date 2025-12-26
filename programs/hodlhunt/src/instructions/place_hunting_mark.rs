use crate::constants::marks;
use crate::errors::ErrorCode;
use crate::Fish;
use crate::{events::*, instructions::PlaceHuntingMark, utils::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program as spl_prog;

/// Charges a hunter for placing an exclusive hunting mark on a prey fish within the
/// permitted hunger window. Verifies mark limits, exclusivity, and hunter ownership
/// before locking the mark and collecting the calculated fee.
pub fn handle(ctx: Context<PlaceHuntingMark>) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let hunter = &mut ctx.accounts.hunter;
    let prey = &mut ctx.accounts.prey;
    let hunter_owner = &mut ctx.accounts.hunter_owner;
    let admin = &mut ctx.accounts.admin;
    let vault = &mut ctx.accounts.vault;
    let system_program = &ctx.accounts.system_program;

    let now = Clock::get()?.unix_timestamp;

    hunter.ensure_alive()?;
    prey.ensure_alive()?;
    hunter.ensure_owned_by(&hunter_owner.key())?;
    require!(hunter.owner != prey.owner, ErrorCode::InvalidPrey);
    require!(hunter.key() != prey.key(), ErrorCode::InvalidPrey);
    require!(hunter.share > prey.share, ErrorCode::PreyTooHeavy);

    let time_until_hungry = (prey.last_fed_at + Fish::PREY_COOLDOWN) - now;
    require!(
        time_until_hungry <= marks::PLACEMENT_WINDOW_SECONDS && time_until_hungry > 0,
        ErrorCode::MarkTooEarly
    );

    prey.clear_expired_mark(now);
    require!(prey.marked_by_hunter_id == 0, ErrorCode::MarkAlreadyActive);

    let prey_value = share_to_value(ocean, prey.share);
    let mark_cost_percent = if time_until_hungry <= marks::HIGH_RATE_THRESHOLD_SECONDS {
        100
    } else {
        50
    };

    let mark_cost_raw = prey_value
        .saturating_mul(mark_cost_percent)
        .saturating_div(1000);

    let min_mark_cost = spl_prog::native_token::LAMPORTS_PER_SOL / 100; // 0.01 SOL
    let mark_cost = if mark_cost_raw < min_mark_cost {
        min_mark_cost
    } else {
        mark_cost_raw
    };
    require!(
        hunter_owner.lamports() >= mark_cost,
        ErrorCode::InsufficientFunds
    );

    let to_pool = mark_cost / 2;
    let to_admin = mark_cost - to_pool;

    let ix_vault =
        spl_prog::system_instruction::transfer(&hunter_owner.key(), &vault.key(), to_pool);
    spl_prog::program::invoke(
        &ix_vault,
        &[
            hunter_owner.to_account_info(),
            vault.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    let ix_admin =
        spl_prog::system_instruction::transfer(&hunter_owner.key(), &admin.key(), to_admin);
    spl_prog::program::invoke(
        &ix_admin,
        &[
            hunter_owner.to_account_info(),
            admin.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    ocean.balance_fishes = ocean.balance_fishes.saturating_add(to_pool);

    prey.marked_by_hunter_id = hunter.id;
    prey.mark_placed_at = now;
    prey.mark_expires_at = prey.last_fed_at + Fish::PREY_COOLDOWN + Fish::MARK_EXCLUSIVITY_PERIOD;
    prey.mark_cost = mark_cost;

    hunter.hunting_marks_placed = hunter.hunting_marks_placed.saturating_add(1);

    emit!(HuntingMarkPlaced {
        mark_id: prey.key(),
        hunter_id: hunter.id,
        prey_id: prey.id,
        hunter_owner: hunter_owner.key(),
        cost: mark_cost,
        expires_at: prey.mark_expires_at,
        time_until_hungry,
        cost_percent: mark_cost_percent,
    });

    Ok(())
}
