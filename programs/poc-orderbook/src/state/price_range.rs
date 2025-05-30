use anchor_lang::prelude::*;

// 2. Price Range Entry in Directory
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PriceRange {
    pub min_price: u64,  // Minimum price in this range (in lamports)
    pub max_price: u64,  // Maximum price in this range (in lamports)
    pub account: Pubkey, // Account address for this price range
}
