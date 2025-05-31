use crate::PriceLevel;
use anchor_lang::prelude::*;

#[account]
pub struct PriceLevelAccount {
    pub authority: Pubkey,             // Same as directory authority
    pub min_price: u64,                // Minimum price in this account (inclusive)
    pub max_price: u64,                // Maximum price in this account (exclusive)
    pub tier_level: u8,                // Tier level of this account
    pub price_levels: Vec<PriceLevel>, // Array of price levels
    pub bump: u8,
}
