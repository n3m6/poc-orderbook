use crate::DirectoryAccount;
use anchor_lang::prelude::*;

// Main orderbook state account
#[account]
pub struct Orderbook {
    pub authority: Pubkey, // Admin authority

    // token accounts
    pub base_mint: Pubkey,  // Token mint for base currency
    pub quote_mint: Pubkey, // Token mint for quote currency

    // orderbook metadata
    pub tick_size: u64,   // Minimum price increment
    pub bucket_size: u64, // Default price range per bucket
    pub best_bid: u64,    // Current best bid price
    pub best_ask: u64,    // Current best ask price
    pub total_bids: u64,  // Total bids in the orderbook
    pub total_asks: u64,  // Total asks in the orderbook

    // directory accounts configuration
    pub max_tier_level: u8, // Maximum tier level to use (eg. 4)

    // TODO add cached pointers to best bid/ask price levels
    // pub best_bid_price_level: Pubkey, // Cached best bid price level account
    // pub best_ask_price_level: Pubkey, // Cached best ask price level account

    // orderbooks
    pub bid_book: DirectoryAccount, // buy book
    pub ask_book: DirectoryAccount, // sell book

    pub bump: u8, // For PDA derivation
}
