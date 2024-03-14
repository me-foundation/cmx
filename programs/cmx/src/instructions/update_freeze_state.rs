use anchor_lang::{prelude::*, Accounts};

use crate::{
    constants::FREEZE_STATE,
    state::{CandyMachine, FreezeState},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFreezeStateArgs {
    pub expiry: i64,
}

#[derive(Accounts)]
pub struct UpdateFreezeState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [FREEZE_STATE.as_bytes(), candy_machine.key().as_ref()],
        payer = authority,
        space = FreezeState::SIZE,
        bump,
    )]
    pub freeze_state: Account<'info, FreezeState>,
    #[account(
        has_one = authority,
    )]
    pub candy_machine: Box<Account<'info, CandyMachine>>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<UpdateFreezeState>, args: UpdateFreezeStateArgs) -> Result<()> {
    let freeze_state = &mut ctx.accounts.freeze_state;
    freeze_state.expiry = args.expiry;

    Ok(())
}
