use crate::Order;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PriceLevel {
    pub price: u64,          // Price level in base currency
    pub total_quantity: u64, // Total quantity at this price
    pub order_count: u32,    // Number of orders at this price
    pub orders: Vec<Order>,  // List of orders at this price
}
