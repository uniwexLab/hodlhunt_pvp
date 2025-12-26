use crate::{errors::ErrorCode, Fish};
use anchor_lang::prelude::*;

/// Validates exclusivity rules for hunting marks, allowing hunts by the mark owner
/// during exclusivity and rejecting others until the window expires.
pub fn check_hunting_mark_exclusivity(
    prey: &mut Fish,
    hunter_id: u64,
    current_time: i64,
) -> Result<()> {
    prey.clear_expired_mark(current_time);

    if prey.marked_by_hunter_id > 0 && prey.mark_placed_at > 0 {
        if prey.marked_by_hunter_id == hunter_id {
            msg!(
                "Hunting with own mark: hunter {} -> prey {}",
                hunter_id,
                prey.id
            );
            return Ok(());
        } else if current_time > prey.mark_expires_at {
            msg!(
                "Hunting after exclusivity period: hunter {} -> prey {} (mark owner: {})",
                hunter_id,
                prey.id,
                prey.marked_by_hunter_id
            );
            return Ok(());
        } else {
            msg!(
                "Mark exclusivity active: only hunter {} can hunt prey {} until {}",
                prey.marked_by_hunter_id,
                prey.id,
                prey.mark_expires_at
            );
            return Err(ErrorCode::MarkExclusivityActive.into());
        }
    }

    msg!(
        "Hunting without mark: hunter {} -> prey {}",
        hunter_id,
        prey.id
    );
    Ok(())
}
