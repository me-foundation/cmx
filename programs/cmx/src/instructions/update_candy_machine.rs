use crate::state::{CandyMachine, LaunchStagesInfo};
use {crate::constants::*, crate::errors::ErrorCode, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct UpdateCandyMachine<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [PREFIX.as_bytes(), candy_machine.config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump
    )]
    pub candy_machine: Account<'info, CandyMachine>,
    #[account(
        mut,
        seeds=[PREFIX.as_bytes(), LAUNCH_STAGES.as_bytes(), candy_machine.key().as_ref()],
        bump=launch_stages_info.bump,
        constraint = launch_stages_info.authority == candy_machine.authority
            && launch_stages_info.candy_machine == candy_machine.key()
            @ ErrorCode::InvalidLaunchStagesInfoFields
    )]
    pub launch_stages_info: Box<Account<'info, LaunchStagesInfo>>,
    pub authority: Signer<'info>,
}

pub fn handle(
    ctx: Context<UpdateCandyMachine>,
    notary: Option<Pubkey>,
    items_available: Option<u64>,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;

    if let Some(notary_address) = notary {
        if notary_address == Pubkey::default() {
            msg!("Notary turned off");
            candy_machine.notary = None;
        } else {
            msg!("Notary changed to {}", notary_address);
            candy_machine.notary = notary;
        }
    }

    if let Some(ia) = items_available {
        let total_redeemed = candy_machine
            .items_redeemed_normal
            .checked_add(candy_machine.raffle_tickets_purchased)
            .ok_or(ErrorCode::NumericalOverflowError)?;
        if ia >= total_redeemed {
            candy_machine.items_available = ia;
        } else {
            msg!("Trying to update candy machine to have {} items when {} items have already been claimed!", ia, candy_machine.items_redeemed_normal + candy_machine.raffle_tickets_purchased);

            return Err(ErrorCode::TooFewItemsAvailable.into());
        }
    }

    Ok(())
}
