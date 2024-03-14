use crate::{errors::ErrorCode, state::Config};
use anchor_lang::{prelude::*, Accounts};

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, has_one = authority)]
    config: Account<'info, Config>,
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<WithdrawFunds<'_>>) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let pay = &ctx.accounts.config.to_account_info();
    let snapshot: u64 = pay.lamports();

    **pay.lamports.borrow_mut() = 0;

    **authority.lamports.borrow_mut() = authority
        .lamports()
        .checked_add(snapshot)
        .ok_or(ErrorCode::NumericalOverflowError)?;

    Ok(())
}
