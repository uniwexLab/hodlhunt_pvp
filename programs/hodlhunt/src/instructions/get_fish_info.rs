use crate::instructions::GetFishInfo;
use anchor_lang::prelude::*;

/// Logs a snapshot of the fish account state for debugging or off-chain inspection.
pub fn handle(ctx: Context<GetFishInfo>) -> Result<()> {
    let fish = &ctx.accounts.fish;

    msg!("Fish Info:");
    msg!("ID: {}", fish.id);
    msg!("Owner: {}", fish.owner);
    msg!("Weight: {} lamports", fish.share);
    msg!("Name: {}", fish.name);
    msg!("Created at: {}", fish.created_at);
    msg!("Last fed at: {}", fish.last_fed_at);
    msg!("Status: {:?}", fish.share > 0);
    msg!("Total hunts: {}", fish.total_hunts);
    msg!("Total hunt income: {} lamports", fish.total_hunt_income);
    msg!("Is protected: {}", fish.is_protected);
    msg!("Protection ends at: {}", fish.protection_ends_at);

    Ok(())
}
