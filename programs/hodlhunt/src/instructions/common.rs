use crate::constants::fees;
use crate::seeds::{derive_name_registry_pda, SEED_NAME};
use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program as spl_prog;

pub struct CreateParams<'info> {
    pub ocean: &'info mut Account<'info, Ocean>,
    pub fish: &'info mut Account<'info, Fish>,
    pub vault: &'info AccountInfo<'info>,
    pub owner: &'info Signer<'info>,
    pub admin: &'info AccountInfo<'info>,
    pub system_program: &'info Program<'info, System>,
}

/// Splits the provided deposit into admin and pool fees, ensuring the payer has
/// sufficient lamports and performing the necessary transfers to the vault and admin.
pub fn apply_creation_finance<'info>(
    owner: &Signer<'info>,
    vault: &AccountInfo<'info>,
    admin: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    deposit: u64,
) -> Result<(u64, u64, u64)> {
    require!(
        deposit >= fees::MIN_DEPOSIT_LAMPORTS,
        crate::errors::ErrorCode::MinimumDeposit
    );
    let admin_fee = deposit / fees::CREATION_FEE_DIVISOR;
    let pool_fee = deposit / fees::CREATION_FEE_DIVISOR;
    let total_cost = deposit + admin_fee + pool_fee;
    require!(
        owner.lamports() >= total_cost,
        crate::errors::ErrorCode::InsufficientFunds
    );

    let ix_vault =
        spl_prog::system_instruction::transfer(&owner.key(), &vault.key(), deposit + pool_fee);
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

    Ok((admin_fee, pool_fee, deposit))
}

/// Mints shares for a new or existing fish by increasing the ocean balance and
/// returning the number of shares granted for the supplied value.
pub fn mint_fish_share(ocean: &mut Ocean, fish: &mut Fish, value: u64) -> u64 {
    ocean.balance_fishes = ocean.balance_fishes.saturating_add(value);
    let share = new_share(ocean, value);
    fish.share = fish.share.saturating_add(share);
    ocean.total_shares = ocean.total_shares.saturating_add(share);
    share
}

/// Initializes core metadata for a newly created fish, setting protection periods,
/// cooldowns, and counters while updating ocean aggregates.
pub fn init_new_fish_meta(ocean: &mut Ocean, fish: &mut Fish, owner: Pubkey, name: &str) {
    fish.id = ocean.next_fish_id;
    fish.owner = owner;
    fish.name = name.to_string();
    let now = Clock::get().unwrap().unix_timestamp;
    fish.created_at = now;
    fish.last_fed_at = now;
    fish.last_hunt_at = now;
    fish.can_hunt_after = now + Fish::CREATION_HUNTING_COOLDOWN;
    fish.is_protected = true;
    fish.protection_ends_at = now + Fish::PROTECTION_PERIOD;
    fish.total_hunts = 0;
    fish.total_hunt_income = 0;
    fish.received_from_hunt_value = 0;
    fish.hunting_marks_placed = 0;
    fish.last_mark_reset = now;
    ocean.total_fish_count = ocean.total_fish_count.saturating_add(1);
    ocean.next_fish_id = ocean.next_fish_id.saturating_add(1);
}

/// Creates a PDA account that tracks reserved fish names, failing if the requested name
/// is invalid, already taken, or the PDA does not match the expected seeds.
pub fn reserve_name_registry<'info>(
    owner: &Signer<'info>,
    name_registry: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    name: &str,
) -> Result<()> {
    let trimmed = name.trim();
    require!(!trimmed.is_empty(), crate::errors::ErrorCode::InvalidName);
    require!(trimmed.len() <= 32, crate::errors::ErrorCode::NameTooLong);
    require!(
        trimmed.chars().all(|c| c.is_ascii() && !c.is_control()),
        crate::errors::ErrorCode::InvalidName
    );

    let (expected_pda, name_hash, bump) =
        derive_name_registry_pda(&crate::ID, trimmed);
    require!(
        name_registry.key() == expected_pda,
        crate::errors::ErrorCode::InvalidName
    );

    if name_registry.lamports() > 0 {
        return Err(crate::errors::ErrorCode::NameAlreadyTaken.into());
    }

    let rent = Rent::get()?.minimum_balance(8u64 as usize);


    if *name_registry.owner == spl_prog::system_program::id() && name_registry.data_len() == 0 {
        let create_ix = spl_prog::system_instruction::create_account(
            &owner.key(),
            &expected_pda,
            rent,
            8,
            &crate::ID,
        );
        spl_prog::program::invoke_signed(
            &create_ix,
            &[
                owner.to_account_info(),
                name_registry.clone(),
                system_program.to_account_info(),
            ],
            &[&[SEED_NAME, name_hash.as_ref(), &[bump]]],
        )?;
    } else if *name_registry.owner == crate::ID && name_registry.data_len() == 8 {
        require!(
            owner.lamports() >= rent,
            crate::errors::ErrorCode::InsufficientFunds
        );

        let transfer_ix = spl_prog::system_instruction::transfer(
            &owner.key(),
            name_registry.key,
            rent,
        );
        spl_prog::program::invoke(
            &transfer_ix,
            &[
                owner.to_account_info(),
                name_registry.clone(),
                system_program.to_account_info(),
            ],
        )?;
    } else {
        return Err(crate::errors::ErrorCode::NameAlreadyTaken.into());
    }

    Ok(())
}

pub fn release_name_if_dead<'info>(
    fish: &Fish,
    name_registry: &AccountInfo<'info>,
    refund_to: &AccountInfo<'info>,
) -> Result<()> {
    if fish.share != 0 {
        return Ok(());
    }

    let (expected_pda, _hash, _bump) = derive_name_registry_pda(&crate::ID, &fish.name);
    require_keys_eq!(*name_registry.key, expected_pda, crate::errors::ErrorCode::InvalidName);

    let lamports = name_registry.lamports();
    if lamports > 0 {
        **refund_to.try_borrow_mut_lamports()? = refund_to
            .lamports()
            .saturating_add(lamports);
        **name_registry.try_borrow_mut_lamports()? = name_registry
            .lamports()
            .saturating_sub(lamports);
    }

    Ok(())
}
