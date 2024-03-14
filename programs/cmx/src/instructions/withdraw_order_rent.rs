use crate::{constants::PREFIX, errors::ErrorCode, state::CandyMachine, state::Order};
use anchor_lang::{prelude::*, Accounts};

#[derive(Accounts)]
pub struct WithdrawOrderRent<'info> {
    #[account(
        seeds = [PREFIX.as_bytes(), candy_machine.config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump,
        constraint = candy_machine.authority == authority.key()
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(
        mut,
        constraint = candy_machine.order_info == order_info.key()
    )]
    order_info: AccountLoader<'info, Order>,
    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<WithdrawOrderRent>) -> Result<()> {
    let authority_ai = &mut ctx.accounts.authority.to_account_info();
    let order_info_ai = &mut ctx.accounts.order_info.to_account_info();
    let candy_machine = &ctx.accounts.candy_machine;
    let curr_redeemed = candy_machine
        .items_redeemed_normal
        .checked_add(candy_machine.items_redeemed_raffle)
        .ok_or(ErrorCode::NumericalOverflowError)?;

    if candy_machine.items_available > curr_redeemed {
        return Err(ErrorCode::MintNotFinished.into());
    }

    let to_transfer_lamp = order_info_ai.lamports();
    let curr_lamp = authority_ai.lamports();
    **order_info_ai.lamports.borrow_mut() = 0;
    **authority_ai.lamports.borrow_mut() = curr_lamp
        .checked_add(to_transfer_lamp)
        .ok_or(ErrorCode::NumericalOverflowError)?;
    Ok(())
}
