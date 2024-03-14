use crate::{
    constants::MAX_ITEMS_AVAILABLE, constants::PREFIX, errors::ErrorCode, state::CandyMachine,
    state::Order,
};
use anchor_lang::{prelude::*, Accounts};

#[derive(Accounts)]
pub struct PopulateOrder<'info> {
    #[account(
        seeds = [PREFIX.as_bytes(), candy_machine.config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump,
        has_one = authority
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(
        mut,
        constraint = candy_machine.order_info == order_info.key()
    )]
    order_info: AccountLoader<'info, Order>,
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<PopulateOrder>, size: u32) -> Result<()> {
    let order_info = &mut ctx.accounts.order_info.load_mut()?;

    let num_left = (MAX_ITEMS_AVAILABLE as u32)
        .checked_sub(order_info.filled)
        .ok_or(ErrorCode::NumericalOverflowError)?;
    let to_fill = std::cmp::min(size, num_left);

    for i in order_info.filled..order_info.filled + to_fill {
        order_info.indices[i as usize] = i
    }

    order_info.filled += to_fill;

    Ok(())
}
