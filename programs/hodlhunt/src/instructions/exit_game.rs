use crate::constants::fees;
use crate::errors::ErrorCode;
use crate::instructions::common::release_name_if_dead;
use crate::{events::*, instructions::ExitGame, utils::*};
use anchor_lang::prelude::*;

/// Allows a fish owner to withdraw from the ocean when conditions permit, distributing
/// exit fees between admin and pool while transferring remaining value to the owner and
/// updating ocean aggregates.
pub fn handle(ctx: Context<ExitGame>) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let fish = &mut ctx.accounts.fish;
    let vault = &mut ctx.accounts.vault;
    let owner = &mut ctx.accounts.owner;
    let admin = &mut ctx.accounts.admin;
    let system_program = &ctx.accounts.system_program;

    fish.ensure_alive()?;
    fish.ensure_owned_by(&owner.key())?;
    require!(!ocean.is_storm, ErrorCode::ExitDuringStorm);

    let total_value = share_to_value(ocean, fish.share);
    let fee_component = total_value
        .saturating_mul(fees::EXIT_FEE_BPS)
        .saturating_div(fees::BASIS_POINTS_DIVISOR);
    let fee_fishes = fee_component;
    let fee_admin = fee_component;
    let withdrawal = total_value
        .saturating_sub(fee_fishes)
        .saturating_sub(fee_admin);

    require!(
        vault.lamports() >= withdrawal,
        ErrorCode::InsufficientVaultBalance
    );

    **vault.try_borrow_mut_lamports()? -= withdrawal;
    **owner.try_borrow_mut_lamports()? += withdrawal;

    if fee_admin > 0 {
        transfer_to_admin(ocean, &ocean.key(), vault, admin, system_program, fee_admin)?;
    }

    ocean.total_shares = ocean
        .total_shares
        .checked_sub(fish.share)
        .ok_or(ErrorCode::MathOverflow)?;
    ocean.balance_fishes = ocean
        .balance_fishes
        .checked_sub(withdrawal)
        .ok_or(ErrorCode::MathOverflow)?;
    ocean.balance_fishes = ocean
        .balance_fishes
        .checked_sub(fee_admin)
        .ok_or(ErrorCode::MathOverflow)?;
    ocean.total_fish_count = ocean
        .total_fish_count
        .checked_sub(1)
        .ok_or(ErrorCode::MathOverflow)?;

    let exited_share = fish.share;
    fish.share = 0;

    release_name_if_dead(fish, &ctx.accounts.name_registry, &owner.to_account_info())?;

    emit!(FishExited {
        fish_id: fish.id,
        owner: fish.owner,
        exited_share,
        payout: total_value,
        admin_fee: fee_admin,
        pool_fee: fee_fishes,
        to_player: withdrawal,
        new_balance: ocean.balance_fishes,
    });

    Ok(())
}
