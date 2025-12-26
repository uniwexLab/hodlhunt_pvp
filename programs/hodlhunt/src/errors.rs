use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // General
    #[msg("Minimum deposit is 0.01 SOL")]
    MinimumDeposit,
    #[msg("Name too long: maximum 32 characters")]
    NameTooLong,
    #[msg("Invalid fish name")]
    InvalidName,
    #[msg("Fish name is already taken")]
    NameAlreadyTaken,
    #[msg("Unauthorized admin action")]
    UnauthorizedAdmin,

    // Ownership / state
    #[msg("Caller is not the fish owner")]
    NotFishOwner,
    #[msg("Fish is already dead")]
    FishAlreadyDead,
    #[msg("Cannot transfer fish to yourself")]
    CannotTransferToSelf,

    // Feeding / funds
    #[msg("Insufficient feeding amount")]
    InsufficientFeedingAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Vault has insufficient balance")]
    InsufficientVaultBalance,
    #[msg("Math overflow/underflow")]
    MathOverflow,

    // Hunting validations
    #[msg("Prey is too heavy")]
    PreyTooHeavy,
    #[msg("Hunter is on hunting cooldown")]
    HuntingOnCooldown,
    #[msg("Invalid prey")]
    InvalidPrey,
    #[msg("Slippage exceeded: prey weight changed more than 5%")]
    SlippageExceeded,

    // Marks
    #[msg("Hunting mark limit exceeded (max 4 per ocean mode period)")]
    MarkLimitExceeded,
    #[msg("Too early to place hunting mark (must be within 3 hours of hunger)")]
    MarkTooEarly,
    #[msg("Hunting mark is inactive")]
    MarkInactive,
    #[msg("Wrong hunter for this mark")]
    MarkWrongHunter,
    #[msg("Wrong prey for this mark")]
    MarkWrongPrey,
    #[msg("Hunting mark has expired")]
    MarkExpired,
    #[msg("Mark exclusivity period active - only mark owner can hunt")]
    MarkExclusivityActive,
    #[msg("An active mark already exists for this prey")]
    MarkAlreadyActive,

    // Exits / ocean
    #[msg("Cannot exit during storm")]
    ExitDuringStorm,
}
