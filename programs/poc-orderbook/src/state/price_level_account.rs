use crate::PriceLevel;
use anchor_lang::prelude::*;

// 3. Price Level Account
#[account]
pub struct PriceLevelAccount {
    pub authority: Pubkey,             // Same as directory authority
    pub min_price: u64,                // Minimum price in this account
    pub max_price: u64,                // Maximum price in this account
    pub price_levels: Vec<PriceLevel>, // Array of price levels
    pub bump: u8,                      // For PDA derivation
}
