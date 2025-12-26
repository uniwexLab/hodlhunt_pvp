use crate::constants::ocean;
use crate::instructions::UpdateOceanDaily;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{hash, hashv};
use anchor_lang::solana_program::sysvar::slot_hashes::SlotHashes;
use anchor_lang::solana_program::sysvar::Sysvar;

/// Advances the ocean's daily cycle when midnight arrives, deriving pseudo-random input
/// from recent chain data to decide whether the mode switches between calm and storm.
pub fn handle(ctx: Context<UpdateOceanDaily>) -> Result<()> {
    let ocean = &mut ctx.accounts.ocean;
    let current_time = Clock::get()?.unix_timestamp;
    msg!(
        "UpdateOceanDaily: now={}, next_change={}",
        current_time,
        ocean.next_mode_change_time
    );

    if !ocean.should_change_mode(current_time) {
        msg!("UpdateOceanDaily: change blocked (waiting for midnight)");
        return Ok(());
    }

    msg!("UpdateOceanDaily: change allowed; proceeding");
    // Build internal entropy buffer and derive a u64 seed via keccak
    let clock = Clock::get()?;
    let slot = clock.slot as u64;
    let mut buf = [0u8; 8 + 8 + 8 + 1];
    buf[0..8].copy_from_slice(&(current_time as u64).to_le_bytes());
    buf[8..16].copy_from_slice(&slot.to_le_bytes());
    buf[16..24].copy_from_slice(&(ocean.cycle_start_time as u64).to_le_bytes());
    buf[24] = ocean.vault_bump;
    // Mix in recent blockhash (via SlotHashes sysvar from accounts)
    let digest = if let Ok(slot_hashes) = SlotHashes::from_account_info(&ctx.accounts.slot_hashes) {
        if let Some((_, recent_hash)) = slot_hashes.slot_hashes().first() {
            hashv(&[&buf, recent_hash.as_ref()])
        } else {
            hash(&buf)
        }
    } else {
        hash(&buf)
    };
    let mut seed_bytes = [0u8; 8];
    seed_bytes.copy_from_slice(&digest.0[..8]);
    let random_seed = u64::from_le_bytes(seed_bytes);
    let new_mode = ocean.determine_next_mode(random_seed);
    let reason = format!("daily_roll_{}bps", ocean::INITIAL_STORM_PROBABILITY_BPS);
    ocean.apply_mode_change(new_mode, current_time, &reason);

    Ok(())
}
