use anchor_lang::prelude::*;

mod instructions;
mod state;

// Bring in the actual instruction handlers and context types

use instructions::*;
use crate::state::ItemCategory;

declare_id!("CvZ7ZXmuGMrCNpTRAnzDZ1ZP6TSWDdDccYbv2sV2jWUQ");

#[program]
pub mod wardrobe_counter {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>, bump: u8) -> Result<()> {
        initialize_handler(ctx, bump)
    }

    pub fn increment(ctx: Context<IncrementUsage>, category: ItemCategory) -> Result<()> {
        increment_handler(ctx, category)
    }
}