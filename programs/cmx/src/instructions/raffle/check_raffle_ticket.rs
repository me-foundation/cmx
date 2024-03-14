use crate::{
    constants::*,
    state::*,
    utils::{get_raffle, is_ticket_winner},
    ErrorCode,
};
use anchor_lang::{prelude::*, Accounts};

#[derive(Accounts)]
pub struct CheckRaffleTicket<'info> {
    config: Box<Account<'info, Config>>,
    #[account(
        has_one = config,
        seeds = [PREFIX.as_bytes(), config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump,
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(
        seeds=[PREFIX.as_bytes(), LAUNCH_STAGES.as_bytes(), candy_machine.key().as_ref()],
        bump=launch_stages_info.bump,
        constraint = launch_stages_info.authority == candy_machine.authority
            && launch_stages_info.candy_machine == candy_machine.key()
            @ ErrorCode::InvalidLaunchStagesInfoFields
    )]
    launch_stages_info: Box<Account<'info, LaunchStagesInfo>>,
    #[account(
        seeds = [
            RAFFLE_TICKET.as_bytes(),
            candy_machine.key().as_ref(),
            payer.key().as_ref(),
        ],
        bump = raffle_ticket.ticket_bump,
    )]
    raffle_ticket: Box<Account<'info, RaffleTicket>>,
    /// CHECK: checked via seeds check above
    payer: UncheckedAccount<'info>,
}

pub fn handle(ctx: Context<CheckRaffleTicket>, curr_time: i64) -> Result<()> {
    let raffle_ticket = &ctx.accounts.raffle_ticket;
    let launch_stages_info = &ctx.accounts.launch_stages_info;

    let raffle_stage = get_raffle(&launch_stages_info.stages)?;
    if curr_time < raffle_stage.end_time {
        return Err(ErrorCode::CurrentStageMismatch.into());
    }

    if let Some(id) = raffle_ticket.ids.last() {
        if is_ticket_winner(*id, &ctx.accounts.candy_machine)? {
            Ok(())
        } else {
            Err(ErrorCode::RaffleTicketNotWinner.into())
        }
    } else {
        Err(ErrorCode::RaffleTicketEmpty.into())
    }
}
