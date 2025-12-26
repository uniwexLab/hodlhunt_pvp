use crate::{instructions::GetNewShare, utils::*};
use anchor_lang::prelude::*;

/// Read-only helper that returns how many shares a deposit would mint in the current ocean state.
pub fn handle(ctx: Context<GetNewShare>, value: u64) -> Result<u64> {
    let ocean = &ctx.accounts.ocean;
    Ok(new_share(ocean, value))
}
