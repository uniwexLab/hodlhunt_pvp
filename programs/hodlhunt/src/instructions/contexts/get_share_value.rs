use crate::state::{Fish, Ocean};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetShareValue<'info> {
    pub ocean: Account<'info, Ocean>,
    pub fish: Account<'info, Fish>,
}
