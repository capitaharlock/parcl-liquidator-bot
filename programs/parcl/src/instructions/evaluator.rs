use anchor_lang::prelude::*;

use crate::errors::LiquidationError;
use crate::state::margin_account::{MarginAccount, Position};
use crate::state::market::Market;

#[derive(Accounts)]
pub struct CheckLiquidationContext<'info> {
    // RJJ + price feed accounts
    // RJJ + market accounts
    #[account(mut)]
    pub margin_account: Account<'info, MarginAccount>,
    pub owner: Signer<'info>,
}

pub fn check_for_liquidation(ctx: Context<CheckLiquidationContext>) -> Result<()> {
    let margin_account = &mut ctx.accounts.margin_account;

    // Early exit if no positions are found
    if margin_account.positions.is_empty() {
        msg!("No positions found in the account.");
        return Err(error!(LiquidationError::NoPositionsFound));
    }

    // Calculate here the required margin amount
    let total_margin_requirement: u64 = margin_account
        .positions
        .iter()
        .map(|position| calculate_margin_requirement(position))
        .sum();

    // RJJ margin updated my main contract according PnL (USDC as a collateral)
    if margin_account.margin < total_margin_requirement {
        msg!("Account margin below required threshold -> liquidate");
        Err(error!(LiquidationError::InsufficientMargin))
        // RJJ (in_liquidation flag is not needed if we do all in one transaction)
        // RJJ-TODO call liquidate function
    } else {
        msg!("Margin requirements ok");
        Ok(())
    }
}

// RJJ-TODO | We may need to read price_feed address from a market account (from id)
#[derive(Accounts)]
pub struct MarketContext<'info> {
    #[account()]
    pub market: Account<'info, Market>,
}

fn calculate_margin_requirement(position: &Position) -> u64 {
    // RJJ add necessary accounts data
    // Get pricefeed | Get from pyth
    let current_market_price: i128 = 0; // RJJ-TODO Read price from pyth | market_id here, needed price_feed account

    // RJJ-TODO | dev
    let initial_ratio = 0; // RJJ-TODO get from MarketSettings
    let min_position_margin = 0; // RJJ-TODO get from MarketSettings
    let maintenance_ratio = 0; // RJJ-TODO check MarketSettings > maintenance_margin_proportion
    let liquidation_fee_rate = 0; // RJJ-TODO get from MarketSettings

    let position_notional = position.size * current_market_price;
    let _initial_margin: i128 = (position_notional * initial_ratio) + min_position_margin;
    let _maintenance_margin: i128 = (position_notional * maintenance_ratio) + min_position_margin;
    let _liquidation_fee_margin: i128 = position_notional * liquidation_fee_rate;

    // Evaluate
    0
}
