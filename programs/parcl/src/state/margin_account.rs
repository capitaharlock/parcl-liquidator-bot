use anchor_lang::prelude::*;

#[account]
pub struct MarginAccount {
    pub positions: Vec<Position>,
    pub margin: u64,
    pub max_liquidation_fee: u64,
    pub id: u32,
    pub exchange: Pubkey,
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub in_liquidation: u8,
    pub bump: u8,
    pub padding: [u8; 10],
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Position {
    pub size: i128,
    pub last_interaction_price: u128,
    pub last_interaction_funding_per_unit: u64,
    pub market_id: u32,
    pub padding: [u8; 4],
}
