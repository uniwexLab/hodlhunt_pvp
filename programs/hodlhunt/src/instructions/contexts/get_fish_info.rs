use crate::state::Fish;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetFishInfo<'info> {
    pub fish: Account<'info, Fish>,
}
