use crate::state::{Fish, Ocean};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PlaceHuntingMark<'info> {
    #[account(mut)]
    pub ocean: Account<'info, Ocean>,

    #[account(mut)]
    pub hunter: Account<'info, Fish>,

    #[account(mut)]
    pub prey: Account<'info, Fish>,

    #[account(
        mut,
        seeds = [b"vault", ocean.key().as_ref()],
        bump = ocean.vault_bump
    )]
    /// CHECK: PDA vault
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub hunter_owner: Signer<'info>,

    #[account(
        mut,
        constraint = admin.key() == ocean.admin
    )]
    /// CHECK: Admin must match ocean.admin
    pub admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
