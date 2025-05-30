use anchor_lang::prelude::*;

// 5. Order
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Order {
    pub owner: Pubkey,  // User who placed the order
    pub quantity: u64,  // Quantity in this order
    pub order_id: u64,  // Unique order ID
    pub timestamp: i64, // When the order was placed
}
