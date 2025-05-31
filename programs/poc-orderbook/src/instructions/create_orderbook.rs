use crate::{DirectoryAccount, Orderbook};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateOrderbook<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Authority who controls the orderbook

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 1, // DirectoryAccount size
        seeds = [b"directory", authority.key().as_ref()],
        bump
    )]
    pub bid_book: Account<'info, DirectoryAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 1, // DirectoryAccount size
        seeds = [b"directory", authority.key().as_ref(), b"ask"],
        bump
    )]
    pub ask_book: Account<'info, DirectoryAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 1, // Orderbook size
        seeds = [b"orderbook", authority.key().as_ref()],
        bump
    )]
    pub orderbook: Account<'info, Orderbook>,

    pub system_program: Program<'info, System>,
}

pub fn create_orderbook(
    ctx: Context<CreateOrderbook>,
    base_mint: Pubkey,  // Base mint  (e.g. USDC)
    quote_mint: Pubkey, // Quote mint (e.g. SOL)
    tick_size: u64,     // Tick size  (e.g. 0.01 USDC)
    current_price: u64, // Current price for the pair
) -> Result<()> {
    
    // TODO directory vector size should be calculated based on tick_size and current_price
    
    ctx.accounts.orderbook.set_inner(Orderbook {
        authority: ctx.accounts.authority.key(),
        base_mint,   // Placeholder, set later
        quote_mint, // Placeholder, set later
        tick_size,   // Example tick size (1 USDC)
        bucket_size: 10_000_000, // Example bucket size (10 USDC)
        best_bid: 0,
        best_ask: 0,
        total_bids: 0,
        total_asks: 0,
        max_tier_level: 4, // Example max tier level
        bid_book: DirectoryAccount {
            authority: ctx.accounts.authority.key(),
            price_ranges: vec![],
            bump: ctx.bumps.bid_book,
        },
        ask_book: DirectoryAccount {
            authority: ctx.accounts.authority.key(),
            price_ranges: vec![],
            bump: ctx.bumps.ask_book,
        },
        bump: ctx.bumps.orderbook,
    });

    msg!(
        "Orderbook created with authority: {}",
        ctx.accounts.orderbook.authority
    );
    Ok(())
}
