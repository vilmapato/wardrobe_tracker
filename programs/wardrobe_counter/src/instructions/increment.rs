use anchor_lang::prelude::*; 
use crate::state::WardrobeCounter;
use crate::state::ItemCategory;

#[derive(Accounts)]
pub struct IncrementUsage<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"wardrobe", authority.key().as_ref()],
        bump = wardrobe_counter.bump,
        has_one = authority
    )]
    pub wardrobe_counter: Account<'info, WardrobeCounter>,
}

pub fn increment_handler(ctx: Context<IncrementUsage>, category: ItemCategory) -> Result<()> {
    let counter = &mut ctx.accounts.wardrobe_counter;

    match category {
        ItemCategory::Clothing => counter.clothing_usage += 1,
        ItemCategory::Shoes => counter.shoe_usage += 1,
        ItemCategory::Accessories => counter.accessory_usage += 1,
    }

    Ok(())
}