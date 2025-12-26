use crate::state::Ocean;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeOcean<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + core::mem::size_of::<Ocean>(),
        seeds = [b"ocean"],
        bump
    )]
    pub ocean: Account<'info, Ocean>,

    #[account(
        init,
        payer = admin,
        space = 0,
        seeds = [b"vault", ocean.key().as_ref()],
        bump
    )]
    /// CHECK: PDA vault
    pub vault: AccountInfo<'info>,

    /// CHECK: Admin to set in ocean state
    pub declared_admin: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
