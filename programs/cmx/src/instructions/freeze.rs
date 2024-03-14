use anchor_lang::{prelude::*, Accounts};
use anchor_spl::{
    self,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::instructions::FreezeDelegatedAccountCpi;

use crate::{
    constants::FREEZE_STATE,
    state::{CandyMachine, FreezeState},
};

#[derive(Accounts)]
pub struct Freeze<'info> {
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [FREEZE_STATE.as_bytes(), candy_machine.key().as_ref()],
        bump,
    )]
    pub freeze_state: Account<'info, FreezeState>,
    pub candy_machine: Box<Account<'info, CandyMachine>>,
    pub asset_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = asset_mint,
        token::authority = payer,
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

pub fn handle(ctx: Context<Freeze>) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let freeze_state = &ctx.accounts.freeze_state;
    let asset_token = &ctx.accounts.asset_token;
    let asset_master_edition = &ctx.accounts.asset_master_edition;
    let asset_mint = &ctx.accounts.asset_mint;
    let payer = &ctx.accounts.payer;

    anchor_spl::token::approve(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Approve {
                to: asset_token.to_account_info(),
                authority: payer.to_account_info(),
                delegate: freeze_state.to_account_info(),
            },
        ),
        asset_token.amount,
    )?;

    FreezeDelegatedAccountCpi {
        __program: &ctx.accounts.token_metadata_program.to_account_info(),
        delegate: &freeze_state.to_account_info(),
        token_account: &asset_token.to_account_info(),
        edition: &asset_master_edition.to_account_info(),
        mint: &asset_mint.to_account_info(),
        token_program: &ctx.accounts.token_program,
    }
    .invoke_signed(&[&[
        FREEZE_STATE.as_bytes(),
        ctx.accounts.candy_machine.key().as_ref(),
        &[*ctx.bumps.get("freeze_state").unwrap()],
    ]])?;

    Ok(())
}
