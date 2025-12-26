use crate::state::Ocean;
use anchor_lang::prelude::*;
// Intentionally use raw AccountInfo to make SlotHashes optional in tests

#[derive(Accounts)]
pub struct UpdateOceanDaily<'info> {
    #[account(mut)]
    pub ocean: Account<'info, Ocean>,
    /// CHECK: Optional SlotHashes sysvar for entropy; fallback used if invalid
    pub slot_hashes: AccountInfo<'info>,
}
