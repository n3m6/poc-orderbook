use crate::PriceRange;
use anchor_lang::prelude::*;

#[account]
pub struct DirectoryAccount {
    pub authority: Pubkey,
    pub price_ranges: Vec<PriceRange>,
    pub bump: u8,
}
