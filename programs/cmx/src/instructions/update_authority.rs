use super::UpdateCandyMachine;
use anchor_lang::prelude::*;

pub fn handle(ctx: Context<UpdateCandyMachine>, new_authority: Option<Pubkey>) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let launch_stages = &mut ctx.accounts.launch_stages_info;

    if let Some(new_auth) = new_authority {
        launch_stages.authority = new_auth;
        candy_machine.authority = new_auth;
    }

    Ok(())
}
