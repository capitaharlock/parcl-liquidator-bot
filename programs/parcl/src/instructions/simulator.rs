//! For test purposes I would like to clone data and create the necessary accounts.
//! It will be nice to have an approximated replica to simulate everything (without having access to main contract)

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

use crate::state::margin_account::MarginAccount;

pub fn clone_margin_account<'info>(ctx: Context<CloneMarginAccountContext<'info>>) -> Result<()> {
    msg!("Starting clone_margin_account function");

    // Check if source account has any data
    if ctx.accounts.source_account.data_len() == 0 {
        msg!("Source length is zero");
        return Err(ProgramError::InvalidAccountData.into());
    }

    // Debug information
    msg!("Source length: {}", ctx.accounts.source_account.data_len());

    // Get data from source account
    msg!("Borrowing data");
    let src_data = &ctx.accounts.source_account.try_borrow_data()?;

    // Unpack data from source account
    let src: MarginAccount = MarginAccount::try_from_slice(src_data)?;
    msg!("Unpacking");

    // Clone account data to destination account
    msg!("Cloning");
    let dest = &mut ctx.accounts.destination_account;
    dest.margin = src.margin;
    dest.max_liquidation_fee = src.max_liquidation_fee;
    dest.id = src.id;
    dest.exchange = src.exchange;
    dest.owner = src.owner;
    dest.delegate = src.delegate;
    dest.in_liquidation = src.in_liquidation;
    dest.bump = src.bump;
    dest.padding = src.padding;

    msg!("Done");
    Ok(())
}

#[derive(Accounts)]
pub struct CloneMarginAccountContext<'info> {
    #[account(
        init,
        payer = user,
        space = 758, // Margin account + 12 positions + d
    )]
    pub destination_account: Account<'info, MarginAccount>,
    /// CHECK: 1
    #[account()]
    pub source_account: AccountInfo<'info>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}
