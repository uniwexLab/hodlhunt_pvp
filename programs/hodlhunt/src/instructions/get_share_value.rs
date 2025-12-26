use crate::{instructions::GetShareValue, utils::*};
use anchor_lang::prelude::*;

/// Read-only helper that converts the specified fish's share balance into lamports.
pub fn handle(ctx: Context<GetShareValue>) -> Result<u64> {
    let ocean = &ctx.accounts.ocean;
    let fish = &ctx.accounts.fish;
    Ok(share_to_value(ocean, fish.share))
}
