use crate::state::margin_account::MarginAccount;
use crate::state::market::Market;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

pub fn liquidate_positions(ctx: Context<LiquidateContext>) -> Result<()> {
    let positions = &mut ctx.accounts.margin_account.positions;

    // RJJ-TODO | get Pyth market price
    let market_price: i128 = 0;

    // Close positions
    for position in positions.iter() {
        let _market_value = market_price * position.size.abs() as i128;

        // RJJ Transfer LP tokens (review doc liquidation on-chain circut)

        // Delete positions (review doc liquidation on-chain circut)
    }

    // RJJ - new approach requires new flag "liquidation_check" instead of "in_liquitation" | RJJ-TODO modify structs needed
    //ctx.accounts.margin_account.liquidation_check = 0; // RJJ 0 will mean no liquidation check

    Ok(())
}

#[derive(Accounts)]
pub struct LiquidateContext<'info> {
    #[account(mut)]
    pub margin_account: Account<'info, MarginAccount>,
    #[account(mut)]
    pub liquidator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub exchange_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub token_program: Program<'info, Token>,
}
