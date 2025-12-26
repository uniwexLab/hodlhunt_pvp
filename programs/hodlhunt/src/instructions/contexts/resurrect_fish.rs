use crate::state::{Fish, Ocean};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String, deposit: u64)]
pub struct ResurrectFish<'info> {
    #[account(mut)]
    pub ocean: Account<'info, Ocean>,

    #[account(mut, has_one = owner)]
    pub old_fish: Account<'info, Fish>,

    #[account(
        init,
        payer = owner,
        space = 8 + Fish::INIT_SPACE,
        seeds = [
            b"fish",
            owner.key().as_ref(),
            &ocean.next_fish_id.to_le_bytes()
        ],
        bump
    )]
    pub new_fish: Account<'info, Fish>,

    /// CHECK: Name registry PDA (unique per name)
    #[account(mut)]
    pub name_registry: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault", ocean.key().as_ref()],
        bump = ocean.vault_bump
    )]
    /// CHECK: PDA vault
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = admin.key() == ocean.admin
    )]
    /// CHECK: Admin must match ocean.admin
    pub admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
