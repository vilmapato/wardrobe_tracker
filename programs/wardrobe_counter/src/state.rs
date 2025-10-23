use anchor_lang::prelude::*;

/// Enum to track item categories
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ItemCategory {
    Clothing,
    Shoes,
    Accessories,
}


#[account]
#[derive(InitSpace)]
pub struct WardrobeCounter {
    pub authority: Pubkey,
    pub clothing_usage: u64,
    pub shoe_usage: u64,
    pub accessory_usage: u64,
    pub bump: u8,
}