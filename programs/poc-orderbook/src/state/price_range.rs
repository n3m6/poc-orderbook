use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PriceRange {
    pub min_price: u64, // Minimum price in this range (inclusive)
    pub max_price: u64, // Maximum price in this range (exclusive)
    pub tier_level: u8,
    pub account: Pubkey, // Account address for this price range
}
