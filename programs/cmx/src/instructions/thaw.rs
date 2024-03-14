use anchor_lang::{prelude::*, solana_program::program_option::COption, Accounts};
use anchor_spl::{
    self,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::instructions::ThawDelegatedAccountCpi;

use crate::{
    constants::FREEZE_STATE,
    errors::ErrorCode,
    state::{CandyMachine, FreezeState},
};

#[derive(Accounts)]
pub struct Thaw<'info> {
    /// CHECK: In case that asset_owner didn't thaw the token account, we can crank this
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: we checked in has_one and also the asset_mint's authority
    #[account(mut)]
    pub asset_owner: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [FREEZE_STATE.as_bytes(), candy_machine.key().as_ref()],
        constraint = freeze_state.expiry == 0 || freeze_state.expiry < Clock::get().unwrap().unix_timestamp @ ErrorCode::CannotThawDueExpiry,
        constraint = asset_token.delegate == COption::Some(freeze_state.key()) @ ErrorCode::InvalidDelegate,
        bump,
    )]
    pub freeze_state: Account<'info, FreezeState>,
    pub candy_machine: Box<Account<'info, CandyMachine>>,
    pub asset_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = asset_owner,
        constraint = asset_mint.supply == 1,
        constraint = asset_mint.decimals == 0,
        constraint = asset_token.amount == 1,
    )]
    pub asset_token: Box<Account<'info, TokenAccount>>,
    /// CHECK: we will check this account in cpi
    pub asset_master_edition: UncheckedAccount<'info>,
    /// CHECK: we will check this account in cpi
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<Thaw>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let freeze_state = &ctx.accounts.freeze_state;
    let asset_token = &ctx.accounts.asset_token;
    let asset_master_edition = &ctx.accounts.asset_master_edition;
    let asset_mint = &ctx.accounts.asset_mint;
    let payer = &ctx.accounts.payer;
    let asset_owner = &ctx.accounts.asset_owner;

    ThawDelegatedAccountCpi {
        __program: &ctx.accounts.token_metadata_program,
        delegate: &freeze_state.to_account_info(),
        token_account: &asset_token.to_account_info(),
        edition: &asset_master_edition.to_account_info(),
        mint: &asset_mint.to_account_info(),
        token_program: &token_program.to_account_info(),
    }
    .invoke_signed(&[&[
        FREEZE_STATE.as_bytes(),
        ctx.accounts.candy_machine.key().as_ref(),
        &[*ctx.bumps.get("freeze_state").unwrap()],
    ]])?;

    // only revoke the delegate if payer is the asset_owner
    if payer.key.eq(asset_owner.key) {
        anchor_spl::token::revoke(CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Revoke {
                source: asset_token.to_account_info(),
                authority: payer.to_account_info(),
            },
        ))?;
    }

    Ok(())
}
