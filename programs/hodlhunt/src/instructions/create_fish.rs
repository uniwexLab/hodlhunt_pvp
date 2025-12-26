use super::common::{
    apply_creation_finance, init_new_fish_meta, mint_fish_share, reserve_name_registry,
};
use crate::{events::*, instructions::CreateFish};
use anchor_lang::prelude::*;

/// Creates a new fish by reserving its name, processing the deposit, minting shares,
/// initializing protection timers, and emitting the corresponding creation event.
pub fn handle(ctx: Context<CreateFish>, name: String, deposit: u64) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let fish = &mut ctx.accounts.fish;
    let vault = &mut ctx.accounts.vault;
    let owner = &mut ctx.accounts.owner;
    let admin = &mut ctx.accounts.admin;
    let system_program = &ctx.accounts.system_program;

    let trimmed = name.trim();

    reserve_name_registry(owner, &ctx.accounts.name_registry, system_program, trimmed)?;

    let (_admin_fee, _pool_fee, _value) =
        apply_creation_finance(owner, vault, admin, system_program, deposit)?;
    ocean.balance_fishes = ocean.balance_fishes.saturating_add(_pool_fee);

    let share = mint_fish_share(ocean, fish, _value);
    init_new_fish_meta(ocean, fish, owner.key(), trimmed);

    emit!(FishCreated {
        fish_id: fish.id,
        owner: fish.owner,
        share,
        deposit: _value,
        admin_fee: _admin_fee,
        pool_fee: _pool_fee,
        name: fish.name.clone(),
    });
    Ok(())
}
