use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MarketSettings {
    pub min_position_margin: u128,
    pub skew_scale: u128,
    pub max_side_size: u128,
    pub max_liquidation_limit_accumulation_multiplier: u64,
    pub max_seconds_in_liquidation_epoch: u64,
    pub initial_margin_ratio: u32,
    pub maker_fee_rate: u16,
    pub taker_fee_rate: u16,
    pub max_funding_velocity: u16,
    pub liquidation_fee_rate: u16,
    pub min_initial_margin_ratio: u16,
    pub maintenance_margin_proportion: u16,
    pub max_liquidation_pd: u16,
    pub authorized_liquidator: Pubkey,
    pub _padding: [u8; 14],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MarketAccounting {
    pub weighted_position_price: u128,
    pub last_utilized_liquidation_capacity: u128,
    pub size: u128,
    pub skew: i128,
    pub weighted_position_funding_per_unit: u128, // RJJ-TODO | Ask if its correct 10^6
    pub last_funding_rate: u128,                  // RJJ-TODO | Ask if its correct 10^6
    pub last_funding_per_unit: u128,              // RJJ-TODO | Ask if its correct 10^6
    pub last_time_funding_updated: u64,
    pub first_liquidation_epoch_start_time: u64,
    pub last_liquidation_epoch_index: u64,
    pub last_time_liquidation_capacity_updated: u64,
    pub _padding: [u8; 8],
}

#[account]
pub struct Market {
    pub settings: MarketSettings,
    pub accounting: MarketAccounting,
    pub id: u32,
    pub exchange: Pubkey,
    pub price_feed: Pubkey,
    pub status: u32,
    pub bump: u8,
    pub _padding: [u8; 10],
}
