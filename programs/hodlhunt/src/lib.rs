#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz");

pub mod events;
pub use events::*;
pub mod errors;
pub mod utils;
pub use utils::*;
pub mod seeds;
pub use seeds::*;
pub mod constants;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;

#[program]
pub mod cryptofish {
    use super::*;

    pub fn initialize_ocean(ctx: Context<InitializeOcean>) -> Result<()> {
        instructions::initialize_ocean::handle(ctx)
    }

    pub fn create_fish(ctx: Context<CreateFish>, name: String, deposit: u64) -> Result<()> {
        instructions::create_fish::handle(ctx, name, deposit)
    }

    pub fn feed_fish(ctx: Context<FeedFish>, feeding_amount: u64) -> Result<()> {
        instructions::feed_fish::handle(ctx, feeding_amount)
    }

    pub fn hunt_fish(ctx: Context<HuntFish>, expected_prey_share: u64) -> Result<()> {
        instructions::hunt_fish::handle(ctx, expected_prey_share)
    }

    pub fn exit_game(ctx: Context<ExitGame>) -> Result<()> {
        instructions::exit_game::handle(ctx)
    }

    pub fn get_fish_info(ctx: Context<GetFishInfo>) -> Result<()> {
        instructions::get_fish_info::handle(ctx)
    }

    pub fn transfer_fish(ctx: Context<TransferFish>) -> Result<()> {
        instructions::transfer_fish::handle(ctx)
    }

    pub fn get_share_value(ctx: Context<GetShareValue>) -> Result<u64> {
        instructions::get_share_value::handle(ctx)
    }

    pub fn get_new_share(ctx: Context<GetNewShare>, value: u64) -> Result<u64> {
        instructions::get_new_share::handle(ctx, value)
    }

    pub fn resurrect_fish(ctx: Context<ResurrectFish>, name: String, deposit: u64) -> Result<()> {
        instructions::resurrect_fish::handle(ctx, name, deposit)
    }

    pub fn place_hunting_mark(ctx: Context<PlaceHuntingMark>) -> Result<()> {
        instructions::place_hunting_mark::handle(ctx)
    }

    pub fn update_ocean_daily(ctx: Context<UpdateOceanDaily>) -> Result<()> {
        instructions::update_ocean_daily::handle(ctx)
    }
}
