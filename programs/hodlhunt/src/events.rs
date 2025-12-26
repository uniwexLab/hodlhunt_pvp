use anchor_lang::prelude::*;

#[event]
pub struct FishFed {
    pub fish_id: u64,
    pub owner: Pubkey,
    pub added_share: u64,
    pub base_cost: u64,
    pub admin_fee: u64,
    pub pool_fee: u64,
    pub new_share: u64,
    pub new_value: u64,
}

#[event]
pub struct FishHunted {
    pub hunter_id: u64,
    pub prey_id: u64,
    pub hunter_owner: Pubkey,
    pub prey_owner: Pubkey,
    pub bite_share: u64,
    pub to_hunter: u64,
    pub to_pool: u64,
    pub to_admin: u64,
    pub enhanced: bool,
    pub hunter_new_share: u64,
    pub prey_new_share: u64,
    pub received_from_hunt_value: u64,
    pub to_admin_value: u64,
    pub to_pool_value: u64,
    pub bite_percent: u64,
    pub bite_fee_percent: u64,
    pub bite_fee: u64,
}

#[event]
pub struct FishExited {
    pub fish_id: u64,
    pub owner: Pubkey,
    pub exited_share: u64,
    pub payout: u64,
    pub admin_fee: u64,
    pub pool_fee: u64,
    pub to_player: u64,
    pub new_balance: u64,
}

#[event]
pub struct FishCreated {
    pub fish_id: u64,
    pub owner: Pubkey,
    pub share: u64,
    pub deposit: u64,
    pub admin_fee: u64,
    pub pool_fee: u64,
    pub name: String,
}

#[event]
pub struct FishTransferred {
    pub fish_id: u64,
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
}

#[event]
pub struct FishResurrected {
    pub old_fish_id: u64,
    pub new_fish_id: u64,
    pub owner: Pubkey,
    pub name: String,
    pub share: u64,
    pub deposit: u64,
    pub admin_fee: u64,
    pub pool_fee: u64,
}

#[event]
pub struct HuntingMarkPlaced {
    pub mark_id: Pubkey,
    pub hunter_id: u64,
    pub prey_id: u64,
    pub hunter_owner: Pubkey,
    pub cost: u64,
    pub expires_at: i64,
    pub time_until_hungry: i64,
    pub cost_percent: u64,
}

#[event]
pub struct OceanModeChanged {
    pub old_mode: bool,
    pub new_mode: bool,
    pub old_feeding_percentage: u16,
    pub new_feeding_percentage: u16,
    pub storm_probability_bps: u16,
    pub cycle_start_time: i64,
    pub next_change_time: i64,
    pub reason: String,
    pub timestamp: i64,
}
