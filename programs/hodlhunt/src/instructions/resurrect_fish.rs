use super::common::{
    apply_creation_finance, init_new_fish_meta, mint_fish_share, reserve_name_registry,
};
use crate::constants::fees;
use crate::errors::ErrorCode;
use crate::{events::*, instructions::ResurrectFish};
use anchor_lang::prelude::*;

/// Revives a previously destroyed fish by reserving its name, processing the deposit
/// with creation fees, minting new shares, and emitting a resurrection event.
pub fn handle(ctx: Context<ResurrectFish>, name: String, deposit: u64) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let old_fish = &mut ctx.accounts.old_fish;
    let new_fish = &mut ctx.accounts.new_fish;
    let vault = &ctx.accounts.vault;
    let owner = &ctx.accounts.owner;
    let admin = &ctx.accounts.admin;
    let system_program = &ctx.accounts.system_program;

    let trimmed = name.trim();

    require!(deposit >= fees::MIN_DEPOSIT_LAMPORTS, ErrorCode::MinimumDeposit);
    require!(owner.lamports() >= deposit, ErrorCode::InsufficientFunds);
    old_fish.ensure_dead()?;

    reserve_name_registry(owner, &ctx.accounts.name_registry, system_program, trimmed)?;

    let (admin_fee, pool_fee, value) =
        apply_creation_finance(owner, vault, admin, system_program, deposit)?;

    ocean.balance_fishes = ocean.balance_fishes.saturating_add(pool_fee);
    let share = mint_fish_share(ocean, new_fish, value);
    init_new_fish_meta(ocean, new_fish, owner.key(), trimmed);

    emit!(FishResurrected {
        old_fish_id: old_fish.id,
        new_fish_id: new_fish.id,
        owner: owner.key(),
        name: trimmed.to_string(),
        share,
        deposit: value,
        admin_fee,
        pool_fee,
    });
    Ok(())
}
