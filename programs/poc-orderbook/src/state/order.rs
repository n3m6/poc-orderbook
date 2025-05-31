use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Order {
    pub owner: Pubkey,
    pub quantity: u64,
    pub order_id: u64,
    pub timestamp: i64,
}
