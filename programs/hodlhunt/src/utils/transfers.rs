use crate::errors::ErrorCode;
use crate::state::Ocean;
use anchor_lang::prelude::*;

/// Moves lamports from the vault PDA to the admin account, ensuring sufficient balance
/// remains and bypassing transfer when the requested amount is zero.
pub fn transfer_to_admin<'a>(
    _ocean: &Ocean,
    _ocean_key: &Pubkey,
    vault: &AccountInfo<'a>,
    admin: &AccountInfo<'a>,
    _system_program: &AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    require!(
        vault.lamports() >= amount,
        ErrorCode::InsufficientFeedingAmount
    );

    **vault.try_borrow_mut_lamports()? -= amount;
    **admin.try_borrow_mut_lamports()? += amount;

    msg!("Transferred {} lamports to admin via CPI", amount);
    Ok(())
}
