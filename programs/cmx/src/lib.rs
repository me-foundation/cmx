#![allow(clippy::result_large_err)]

pub mod constants;
pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use {
    crate::errors::ErrorCode,
    crate::instructions::*,
    crate::state::*,
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
};
anchor_lang::declare_id!("CMZYPASGWeTz7RNGHaRJfCq2XQ5pYK6nDvVQxzkH51zb");

#[program]
pub mod cmx {
    use super::*;

    pub fn mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
        wallet_limit_bump: u8,
        in_order: bool,
        user_limit: Option<u8>,
        curr_time: i64,
    ) -> Result<()> {
        instructions::mint_nft::handle(ctx, wallet_limit_bump, in_order, user_limit, curr_time)
    }

    pub fn mint_nft_mip1<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFTMIP1<'info>>,
        wallet_limit_bump: u8,
        in_order: bool,
        user_limit: Option<u8>,
        curr_time: i64,
    ) -> Result<()> {
        instructions::mint_nft_mip1::handle(ctx, wallet_limit_bump, in_order, user_limit, curr_time)
    }

    pub fn buy_raffle_ticket(
        ctx: Context<BuyRaffleTicket>,
        wallet_limit_bump: u8,
        raffle_ticket_bump: u8,
        escrow_bump: u8,
        curr_time: i64,
    ) -> Result<()> {
        instructions::buy_raffle_ticket::handle(
            ctx,
            wallet_limit_bump,
            raffle_ticket_bump,
            escrow_bump,
            curr_time,
        )
    }

    pub fn check_raffle_ticket(ctx: Context<CheckRaffleTicket>, curr_time: i64) -> Result<()> {
        instructions::check_raffle_ticket::handle(ctx, curr_time)
    }

    // settle ONE ticket, we need to call multiple times to settle multiple tickets!
    pub fn settle_raffle_ticket(ctx: Context<SettleRaffleTicket>, curr_time: i64) -> Result<()> {
        instructions::settle_raffle_ticket::handle(ctx, curr_time)
    }

    pub fn update_candy_machine(
        ctx: Context<UpdateCandyMachine>,
        notary: Option<Pubkey>,
        items_available: Option<u64>,
    ) -> Result<()> {
        instructions::update_candy_machine::handle(ctx, notary, items_available)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        args: InitializeConfigArgs,
    ) -> Result<()> {
        instructions::initialize_config::handle(ctx, args)
    }

    pub fn initialize_candy_machine(
        ctx: Context<InitializeCandyMachine>,
        args: InitializeCandyMachineArgs,
    ) -> Result<()> {
        instructions::initialize_candy_machine::handle(ctx, args)
    }

    pub fn update_authority(
        ctx: Context<UpdateCandyMachine>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        instructions::update_authority::handle(ctx, new_authority)
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFunds<'_>>) -> Result<()> {
        instructions::withdraw_funds::handle(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigArgs) -> Result<()> {
        instructions::update_config::handle(ctx, params)
    }

    pub fn update_launch_stages(
        ctx: Context<UpdateLaunchStages>,
        stages: Vec<LaunchStageArgs>,
        curr_time: i64,
        notary_required: Vec<bool>,
    ) -> Result<()> {
        instructions::update_launch_stages::handle(ctx, stages, curr_time, notary_required)
    }

    pub fn unverify_non_master_edition(ctx: Context<UnverifyNonMasterEdition>) -> Result<()> {
        instructions::unverify_non_master_edition::handle(ctx)
    }

    pub fn populate_order(ctx: Context<PopulateOrder>, size: u32) -> Result<()> {
        instructions::populate_order::handle(ctx, size)
    }

    pub fn withdraw_order_rent(ctx: Context<WithdrawOrderRent>) -> Result<()> {
        instructions::withdraw_order_rent::handle(ctx)
    }

    pub fn freeze(ctx: Context<Freeze>) -> Result<()> {
        instructions::freeze::handle(ctx)
    }

    pub fn thaw(ctx: Context<Thaw>) -> Result<()> {
        instructions::thaw::handle(ctx)
    }

    // update uses init_if_needed for upsert
    pub fn update_freeze_state(
        ctx: Context<UpdateFreezeState>,
        args: UpdateFreezeStateArgs,
    ) -> Result<()> {
        instructions::update_freeze_state::handle(ctx, args)
    }
}
