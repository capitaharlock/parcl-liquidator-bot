use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

use solana_security_txt::security_txt;

declare_id!("EoMpzp7ZvyvbGWBNzWPTCxEfsEPYgiKkeAi6HiS4vWv3");

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
}
