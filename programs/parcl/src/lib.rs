use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

use solana_security_txt::security_txt;

declare_id!("7tymZxRUuwEqAsqmhs4BaCG4PXhTN8DfSeYAsoLboc4r");

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
        margin: u64,
        max_liquidation_fee: u64,
        exchange: Pubkey,
        delegate: Pubkey,
        in_liquidation: u8,
    ) -> Result<()> {
        instructions::set_margin_account(
            ctx,
            id,
            positions,
            margin,
            max_liquidation_fee,
            exchange,
            delegate,
            in_liquidation,
        )
    }

    // Simulator | modify margin account
    pub fn put_margin_account(
        ctx: Context<PutMarginAccountContext>,
        id: u32,
        positions: Option<Vec<Position>>,
        margin: Option<u64>,
        max_liquidation_fee: Option<u64>,
        in_liquidation: Option<u8>,
    ) -> Result<()> {
        instructions::put_margin_account(
            ctx,
            id,
            positions,
            margin,
            max_liquidation_fee,
            in_liquidation,
        )
    }

    // Simulator | close margin account
    pub fn delete_margin_account(ctx: Context<DeleteMarginAccountContext>, id: u32) -> Result<()> {
        instructions::delete_margin_account(ctx, id)
    }
}
