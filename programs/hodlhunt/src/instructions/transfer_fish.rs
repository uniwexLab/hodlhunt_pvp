use crate::errors::ErrorCode;
use crate::instructions::TransferFish;
use anchor_lang::prelude::*;

/// Moves a fish account to a new owner after validating ownership, liveliness,
/// and preventing self-transfers. Copies the existing fish state into the
/// destination account and emits a transfer event.
pub fn handle(ctx: Context<TransferFish>) -> Result<()> {
    let fish = &ctx.accounts.fish;

    fish.ensure_owned_by(&ctx.accounts.current_owner.key())?;
    require!(
        ctx.accounts.current_owner.key() != ctx.accounts.new_owner.key(),
        ErrorCode::CannotTransferToSelf
    );
    fish.ensure_alive()?;

    let new_fish = &mut ctx.accounts.new_fish;
    // Full state copy (except owner which is set to the new owner)
    new_fish.id = fish.id;
    new_fish.owner = ctx.accounts.new_owner.key();
    new_fish.name = fish.name.clone();
    new_fish.share = fish.share;
    new_fish.created_at = fish.created_at;
    new_fish.last_fed_at = fish.last_fed_at;
    new_fish.last_hunt_at = fish.last_hunt_at;
    new_fish.can_hunt_after = fish.can_hunt_after;
    new_fish.is_protected = fish.is_protected;
    new_fish.protection_ends_at = fish.protection_ends_at;
    new_fish.total_hunts = fish.total_hunts;
    new_fish.total_hunt_income = fish.total_hunt_income;
    new_fish.received_from_hunt_value = fish.received_from_hunt_value;
    new_fish.hunting_marks_placed = fish.hunting_marks_placed;
    new_fish.last_mark_reset = fish.last_mark_reset;
    new_fish.marked_by_hunter_id = fish.marked_by_hunter_id;
    new_fish.mark_placed_at = fish.mark_placed_at;
    new_fish.mark_expires_at = fish.mark_expires_at;
    new_fish.mark_cost = fish.mark_cost;

    // Emit transfer event
    emit!(crate::FishTransferred {
        fish_id: new_fish.id,
        from_owner: ctx.accounts.current_owner.key(),
        to_owner: ctx.accounts.new_owner.key(),
    });

    Ok(())
}
