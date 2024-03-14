use crate::{
    constants::PREFIX,
    errors::ErrorCode,
    state::{CandyMachine, LaunchStage, LaunchStageArgs, LaunchStagesInfo},
    utils::assert_stages,
};
use anchor_lang::{prelude::*, Accounts};
use anchor_spl::{associated_token, token::Token};

#[derive(Accounts)]
pub struct UpdateLaunchStages<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [PREFIX.as_bytes(), candy_machine.config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(mut, has_one = authority)]
    launch_stages_info: Account<'info, LaunchStagesInfo>,
    authority: Signer<'info>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handle(
    ctx: Context<UpdateLaunchStages>,
    stages: Vec<LaunchStageArgs>,
    curr_time: i64,
    notary_required: Vec<bool>,
) -> Result<()> {
    let launch_stages_info = &mut ctx.accounts.launch_stages_info;
    let candy_machine = &mut ctx.accounts.candy_machine;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let token_program = &ctx.accounts.token_program;
    let num_rem_accounts = ctx.remaining_accounts.len();
    if num_rem_accounts % 2 == 1 {
        return Err(ErrorCode::AtaMismatch.into());
    }
    let mint_ai = &ctx.remaining_accounts[0..num_rem_accounts / 2];
    let ata_ai = &ctx.remaining_accounts[num_rem_accounts / 2..num_rem_accounts];
    let old_stages = &launch_stages_info.stages;

    if stages.len() < old_stages.len() && curr_time > old_stages[0].start_time {
        return Err(ErrorCode::CannotDeleteStages.into());
    }
    let mut new_stages: Vec<LaunchStage> = stages
        .iter()
        .map(|stage_arg| LaunchStage {
            stage_type: stage_arg.stage_type,
            start_time: stage_arg.start_time,
            end_time: stage_arg.end_time,
            wallet_limit: stage_arg.wallet_limit,
            price: stage_arg.price,
            stage_supply: stage_arg.stage_supply,
            previous_stage_unminted_supply: 0,
            minted_during_stage: 0,
            payment_mint: *mint_ai[stage_arg.payment_mint_index as usize].key,
            payment_ata: *ata_ai[stage_arg.payment_mint_index as usize].key,
        })
        .collect();

    assert_stages(
        &stages,
        mint_ai,
        ata_ai,
        token_program.key,
        associated_token_program.key,
        &candy_machine.wallet_authority,
        candy_machine.items_available,
        &notary_required,
    )?;

    // fill out previous_stage_supply_overflow and minted_during_stage
    new_stages[0].minted_during_stage = old_stages[0].minted_during_stage;
    let to_iter = std::cmp::min(old_stages.len(), new_stages.len());
    for i in 1..to_iter {
        if curr_time > new_stages[i].start_time {
            match new_stages[i - 1].stage_supply {
                Some(l) => {
                    let prev_supply = l
                        .checked_add(new_stages[i - 1].previous_stage_unminted_supply)
                        .ok_or(ErrorCode::NumericalOverflowError)?;
                    let to_sub = std::cmp::max(new_stages[i - 1].minted_during_stage, prev_supply);
                    new_stages[i].previous_stage_unminted_supply = to_sub
                        .checked_sub(new_stages[i - 1].minted_during_stage)
                        .ok_or(ErrorCode::NumericalOverflowError)?;
                }
                None => {
                    // we don't allow overflow from stages with no limit on supply
                    new_stages[i].previous_stage_unminted_supply = 0;
                }
            }
        }

        new_stages[i].minted_during_stage = old_stages[i].minted_during_stage;
    }

    if !candy_machine.is_lite && !notary_required.iter().all(|v| *v) {
        return Err(ErrorCode::NonLiteCandyMachineInvalidNotaryRequired.into());
    }

    launch_stages_info.stages = new_stages;
    candy_machine.notary_required = notary_required;

    Ok(())
}
