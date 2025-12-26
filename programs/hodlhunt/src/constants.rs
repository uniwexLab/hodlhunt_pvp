pub mod ocean {
    pub const DAY_DURATION: i64 = 24 * 60 * 60;
    pub const CALM_FEEDING_BPS: u16 = 500; // 5%
    pub const STORM_FEEDING_BPS: u16 = 1000; // 10%
    pub const INITIAL_STORM_PROBABILITY_BPS: u16 = 250; // 25%
}

pub mod fees {
    pub const MIN_DEPOSIT_LAMPORTS: u64 = 10_000_000; // 0.01 SOL
    pub const MIN_FEED_LAMPORTS: u64 = 10_000_000; // 0.01 SOL
    pub const FEED_COMMISSION_DIVISOR: u64 = 10; // 10%
    pub const FEE_SPLIT_DIVISOR: u64 = 2; // 50/50 
    pub const CREATION_FEE_DIVISOR: u64 = 20; // 5% from deposit
    pub const BASIS_POINTS_DIVISOR: u64 = 10_000;
    pub const EXIT_FEE_BPS: u64 = 500; // 5%
}

pub mod marks {
    pub const PLACEMENT_WINDOW_SECONDS: i64 = 3 * 60 * 60; // 3 hours
    pub const HIGH_RATE_THRESHOLD_SECONDS: i64 = 30 * 60; // 30 minutes
    pub const EXCLUSIVITY_SECONDS: i64 = 20 * 60; // 20 minutes
}
