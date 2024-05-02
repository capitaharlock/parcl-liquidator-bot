//! For test purposes I would like to clone data and create the necessary accounts.
//! It will be nice to have an approximated replica to simulate everything (without having access to main contract)

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

use crate::errors::AuthError;
use crate::state::margin_account::MarginAccount;
use crate::state::margin_account::Position;

// Clone data from Parcl
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

// Generate Margin Account simulated data
pub fn set_margin_account<'info>(
    ctx: Context<SetMarginAccountContext<'info>>,
    id: u32,
    positions: Vec<Position>,
    margin: u64,
    max_liquidation_fee: u64,
    exchange: Pubkey,
    delegate: Pubkey,
    in_liquidation: u8,
) -> Result<()> {
    let margin_account = &mut ctx.accounts.margin_account;
    let user = &ctx.accounts.user.key();

    // Save data
    margin_account.id = id;
    margin_account.positions = positions;
    margin_account.margin = margin;
    margin_account.max_liquidation_fee = max_liquidation_fee;
    margin_account.exchange = exchange;
    margin_account.delegate = delegate;
    margin_account.in_liquidation = in_liquidation;
    margin_account.owner = *user;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u32)]
pub struct SetMarginAccountContext<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 756, // data + 13 positions (add more in case that more postions are needed in a near future)
        seeds = [
            b"margin_account",
            exchange.key().as_ref(),
            user.key().as_ref(),
            &id.to_le_bytes(),
        ],
        bump,
    )]
    pub margin_account: Account<'info, MarginAccount>,
    /// CHECK: exchange address as seed only
    exchange: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Update Margin Account simulated data
pub fn put_margin_account<'info>(
    ctx: Context<PutMarginAccountContext<'info>>,
    id: u32,
    positions: Option<Vec<Position>>,
    margin: Option<u64>,
    max_liquidation_fee: Option<u64>,
    in_liquidation: Option<u8>,
) -> Result<()> {
    let user = ctx.accounts.user.key();
    let margin_account = &mut ctx.accounts.margin_account;

    // Security - testing here mods allowed by a delegate signer or owner
    require!(
        user == margin_account.owner || user == margin_account.delegate,
        AuthError::Unauthorized
    );

    /*
    Positions and margin management
    - Reducing margin -> return collateral
    - Clean positions (use flag)
    - Move margin login to another fn
    */

    // Save data
    if let Some(positions) = positions {
        margin_account.positions = positions;
    }
    if let Some(margin) = margin {
        margin_account.margin = margin;
    }
    if let Some(max_liquidation_fee) = max_liquidation_fee {
        margin_account.max_liquidation_fee = max_liquidation_fee;
    }
    if let Some(in_liquidation) = in_liquidation {
        margin_account.in_liquidation = in_liquidation;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u32)]
pub struct PutMarginAccountContext<'info> {
    #[account(
        mut,
        seeds = [
            b"margin_account",
            exchange.key().as_ref(),
            user.key().as_ref(),
            &id.to_le_bytes(),
        ],
        bump,
    )]
    pub margin_account: Account<'info, MarginAccount>,
    /// CHECK: exchange address as seed only
    exchange: AccountInfo<'info>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

// Close margin account
pub fn delete_margin_account<'info>(
    ctx: Context<DeleteMarginAccountContext<'info>>,
    _id: u32,
) -> Result<()> {
    let user = ctx.accounts.user.key();
    let margin_account = &mut ctx.accounts.margin_account;

    // Security - testing here mods allowed by a delegate signer or owner
    require!(
        user == margin_account.owner || user == margin_account.delegate,
        AuthError::Unauthorized
    );

    // Return margin to the user

    // We're done. If succeded all lamports will be returned to the owner.
    margin_account.margin = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u32)]
pub struct DeleteMarginAccountContext<'info> {
    #[account(
        mut,
        seeds = [
            b"margin_account",
            exchange.key().as_ref(),
            user.key().as_ref(),
            &id.to_le_bytes(),
        ],
        bump,
        close = user,
    )]
    pub margin_account: Account<'info, MarginAccount>,
    /// CHECK: exchange address as seed only
    exchange: AccountInfo<'info>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

/*
Audit:
no reinit
ownner
 */
