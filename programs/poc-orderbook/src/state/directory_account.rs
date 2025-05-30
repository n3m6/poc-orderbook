use crate::PriceRange;
use anchor_lang::prelude::*;

// 1. Directory Account
#[account]
pub struct DirectoryAccount {
    pub authority: Pubkey,             // Who controls this orderbook
    pub price_ranges: Vec<PriceRange>, // List of price ranges and their accounts
    pub bump: u8,                      // For PDA derivation
}
