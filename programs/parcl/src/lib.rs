use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

use solana_security_txt::security_txt;

declare_id!("9zp54Krm3Sy5xT11ahDefb3LUszYhc5m8aDbYe9pMUVa");

security_txt! {
    // Required fields
    name: "Parcl liquidator bot",
    project_url: "https://parcl.co/",
    contacts: "email:none",
    policy: "",
    preferred_languages: "en",
    source_code: "https://github.com/capitaharlock/parcl-liquidator-bot"
}

#[program]
pub mod parcl {
    use super::*;

    // Evaluate | Check if margin is up to liquidate
    pub fn check_for_liquidation(ctx: Context<CheckLiquidationContext>) -> Result<()> {
        instructions::evaluator::check_for_liquidation(ctx)
    }

    // Simulator | clone a margin account
    pub fn clone_margin_account_entry(ctx: Context<CloneMarginAccountContext>) -> Result<()> {
        instructions::clone_margin_account(ctx)
    }

    // Simulator | create margin account
    pub fn set_margin_account(
        ctx: Context<SetMarginAccountContext>,
        id: u32,
        positions: Vec<Position>,
    ) -> Result<()> {
        instructions::set_margin_account(ctx, id, positions)
    }

    // Simulator | modify margin account
    pub fn put_margin_account(
        ctx: Context<PutMarginAccountContext>,
        id: u32,
        positions: Vec<Position>,
    ) -> Result<()> {
        instructions::put_margin_account(ctx, id, positions)
    }

    // Simulator | close margin account
    pub fn delete_margin_account(ctx: Context<DeleteMarginAccountContext>, id: u32) -> Result<()> {
        instructions::delete_margin_account(ctx, id)
    }
}
