use crate::{
    constants::*,
    errors::ErrorCode,
    state::{CandyMachine, Config, LaunchStage, LaunchStagesInfo},
    state::{LaunchStageArgs, Order},
    utils::assert_stages,
};
use anchor_lang::{prelude::*, Accounts};
use anchor_spl::{associated_token, token::Token};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeCandyMachineArgs {
    pub cm_bump: u8,
    pub launch_stages_bump: u8,
    pub uuid: String,
    pub items_available: u64,
    pub stages: Vec<LaunchStageArgs>,
    pub is_lite: bool,
    pub notary_required: Vec<bool>,
    pub mip1_ruleset: Option<Pubkey>,
    pub is_open_edition: Option<bool>,
}

#[derive(Accounts)]
#[instruction(args: InitializeCandyMachineArgs)]
pub struct InitializeCandyMachine<'info> {
    #[account(
        init,
        seeds=[PREFIX.as_bytes(), config.key().as_ref(), args.uuid.as_bytes()],
        payer=payer,
        bump,
        space=CandyMachine::SIZE
    )]
    candy_machine: Box<Account<'info, CandyMachine>>,
    #[account(
        init,
        seeds=[PREFIX.as_bytes(), LAUNCH_STAGES.as_bytes(), candy_machine.key().as_ref()],
        payer=payer,
        bump,
        space=LaunchStagesInfo::SIZE
    )]
    launch_stages_info: Box<Account<'info, LaunchStagesInfo>>,
    #[account(zero)]
    order_info: AccountLoader<'info, Order>,
    #[account(constraint = (wallet_authority.data_is_empty() && wallet_authority.lamports() > 0) )]
    /// CHECK: checked above
    wallet_authority: UncheckedAccount<'info>,
    #[account(has_one=authority)]
    config: Box<Account<'info, Config>>,
    #[account(constraint= authority.data_is_empty() && authority.lamports() > 0)]
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: should be an empty account that only signs
    notary: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handle(
    ctx: Context<InitializeCandyMachine>,
    args: InitializeCandyMachineArgs,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let launch_stages_info = &mut ctx.accounts.launch_stages_info;
    let order_info = &mut ctx.accounts.order_info.load_init()?;
    let num_rem_accounts = ctx.remaining_accounts.len();
    if num_rem_accounts % 2 == 1 {
        return Err(ErrorCode::AtaMismatch.into());
    }
    let mint_ai = &ctx.remaining_accounts[0..num_rem_accounts / 2];
    let ata_ai = &ctx.remaining_accounts[num_rem_accounts / 2..num_rem_accounts];

    if args.uuid.len() != 6 {
        return Err(ErrorCode::UuidMustBeExactly6Length.into());
    }
    assert_stages(
        &args.stages,
        mint_ai,
        ata_ai,
        ctx.accounts.token_program.key,
        ctx.accounts.associated_token_program.key,
        ctx.accounts.wallet_authority.key,
        args.items_available,
        &args.notary_required,
    )?;

    let order_info_key = ctx.accounts.order_info.key();

    order_info.filled = 0;
    order_info.candy_machine = candy_machine.key();

    launch_stages_info.bump = args.launch_stages_bump;
    launch_stages_info.authority = *ctx.accounts.authority.key; // same authority as candy machine
    launch_stages_info.candy_machine = candy_machine.key();

    launch_stages_info.stages = args
        .stages
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

    candy_machine.uuid = args.uuid;
    candy_machine.items_available = args.items_available;
    candy_machine.wallet_authority = *ctx.accounts.wallet_authority.key;
    candy_machine.authority = *ctx.accounts.authority.key;
    candy_machine.config = ctx.accounts.config.key();
    candy_machine.bump = args.cm_bump;
    candy_machine.raffle_seed = 0;
    candy_machine.order_info = order_info_key;
    candy_machine.is_lite = args.is_lite;
    candy_machine.mip1_ruleset = args.mip1_ruleset;
    candy_machine.is_open_edition = args.is_open_edition;

    if !args.is_lite && !args.notary_required.iter().all(|v| *v) {
        return Err(ErrorCode::NonLiteCandyMachineInvalidNotaryRequired.into());
    }
    candy_machine.notary_required = args.notary_required;

    if ctx.accounts.notary.key() != Pubkey::default() {
        candy_machine.notary = Some(ctx.accounts.notary.key());
    }

    Ok(())
}
