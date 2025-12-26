use crate::constants::fees;
use crate::errors::ErrorCode;
use crate::{events::*, instructions::FeedFish, utils::*, Fish};
use anchor_lang::prelude::*;
use anchor_lang::solana_program as spl_prog;

/// Transfers the feeding payment, applies commissions, updates share balances and marks
/// the fish as recently fed. Resets hunt-related flags and enforces minimum feeding amounts
/// derived from the current ocean state and previous hunt rewards.
pub fn handle(ctx: Context<FeedFish>, feeding_amount: u64) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let fish = &mut ctx.accounts.fish;
    let vault = &mut ctx.accounts.vault;
    let owner = &mut ctx.accounts.owner;
    let admin = &mut ctx.accounts.admin;
    let system_program = &ctx.accounts.system_program;

    fish.ensure_alive()?;
    fish.ensure_owned_by(&owner.key())?;

    let min_required_cost = min_feeding_amount(ocean, fish);

    require!(
        feeding_amount >= min_required_cost,
        ErrorCode::InsufficientFeedingAmount
    );

    let commission = feeding_amount / fees::FEED_COMMISSION_DIVISOR;
    let admin_fee = commission / fees::FEE_SPLIT_DIVISOR;
    let pool_fee = commission - admin_fee;
    let total_cost = feeding_amount + commission;

    require!(owner.lamports() >= total_cost, ErrorCode::InsufficientFunds);

    let ix_vault = spl_prog::system_instruction::transfer(
        &owner.key(),
        &vault.key(),
        feeding_amount + pool_fee,
    );
    spl_prog::program::invoke(
        &ix_vault,
        &[
            owner.to_account_info(),
            vault.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    let ix_admin = spl_prog::system_instruction::transfer(&owner.key(), &admin.key(), admin_fee);
    spl_prog::program::invoke(
        &ix_admin,
        &[
            owner.to_account_info(),
            admin.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    ocean.balance_fishes = ocean
        .balance_fishes
        .saturating_add(feeding_amount + pool_fee);

    let added_share = new_share(ocean, feeding_amount);
    fish.share = fish.share.saturating_add(added_share);
    ocean.total_shares = ocean.total_shares.saturating_add(added_share);

    let now = Clock::get()?.unix_timestamp;
    fish.last_fed_at = now;
    fish.marked_by_hunter_id = 0;
    fish.mark_placed_at = 0;
    fish.mark_expires_at = 0;
    fish.mark_cost = 0;
    fish.can_hunt_after = now + Fish::FEEDING_COOLDOWN;
    fish.received_from_hunt_value = 0;

    emit!(FishFed {
        fish_id: fish.id,
        owner: fish.owner,
        added_share,
        base_cost: feeding_amount,
        admin_fee,
        pool_fee,
        new_share: fish.share,
        new_value: share_to_value(ocean, fish.share),
    });
    Ok(())
}
