use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::{hash, Hash};

pub const SEED_OCEAN: &[u8] = b"ocean";
pub const SEED_VAULT: &[u8] = b"vault";
pub const SEED_FISH: &[u8] = b"fish";
pub const SEED_NAME: &[u8] = b"fish_name";

/// Derives the vault PDA associated with the provided ocean account.
pub fn derive_vault_pda(program_id: &Pubkey, ocean: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SEED_VAULT, ocean.as_ref()], program_id)
}

/// Derives the fish PDA for the specified owner and fish identifier.
pub fn derive_fish_pda(program_id: &Pubkey, owner: &Pubkey, fish_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SEED_FISH, owner.as_ref(), &fish_id.to_le_bytes()], program_id)
}

/// Derives the name registry PDA and its hash seed for the provided name string.
pub fn derive_name_registry_pda(program_id: &Pubkey, name: &str) -> (Pubkey, Hash, u8) {
    let name_hash = hash(name.as_bytes());
    let (pda, bump) = Pubkey::find_program_address(&[SEED_NAME, name_hash.as_ref()], program_id);
    (pda, name_hash, bump)
}
