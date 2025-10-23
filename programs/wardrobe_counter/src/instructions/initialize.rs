use anchor_lang::prelude::*;

use crate::state::WardrobeCounter;

#[derive(Accounts)]

pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"wardrobe", authority.key().as_ref()],
        bump
    )]
    pub wardrobe_counter: Account<'info, WardrobeCounter>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<InitializeCounter>, bump: u8) -> Result<()> {
    let counter = &mut ctx.accounts.wardrobe_counter;
    counter.authority = ctx.accounts.authority.key();
    counter.clothing_usage = 0;
    counter.shoe_usage = 0;
    counter.accessory_usage = 0;
    counter.bump = bump;
    Ok(())
}