use crate::state::Ocean;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetNewShare<'info> {
    pub ocean: Account<'info, Ocean>,
}
