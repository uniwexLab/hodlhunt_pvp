use crate::state::{Fish, Ocean};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferFish<'info> {
    /// Ocean state (not modified, but client sends it first; keep mut to be flexible)
    #[account(mut)]
    pub ocean: Account<'info, Ocean>,

    /// The original fish account to transfer from (will be closed to current_owner)
    #[account(mut, close = current_owner)]
    pub fish: Account<'info, Fish>,

    /// New fish account initialized for the new owner with the same fish id
    #[account(
        init,
        payer = current_owner,
        space = 8 + Fish::INIT_SPACE,
        seeds = [
            b"fish",
            new_owner.key().as_ref(),
            &fish.id.to_le_bytes()
        ],
        bump
    )]
    pub new_fish: Account<'info, Fish>,

    /// Current owner who authorizes the transfer and receives rent from the closed account
    #[account(mut)]
    pub current_owner: Signer<'info>,

    /// New owner who will receive the fish (no signature required)
    /// CHECK: not a signer by design; validated in instruction logic
    #[account(mut)]
    pub new_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
