use crate::{
    constants::*,
    state::*,
    utils::{
        assert_ata_address, assert_initialized, get_current_stage_index, get_wallet_limit_info,
        spl_sync_native, spl_token_transfer, TokenTransferParams,
    },
    ErrorCode,
};
use anchor_lang::solana_program::hash::hashv;
use anchor_lang::{prelude::*, solana_program::sysvar, Accounts, Discriminator};
use anchor_spl::{
    self, associated_token,
    token::{close_account, CloseAccount, Mint, Token, TokenAccount},
};
use arrayref::array_ref;
use spl_token::native_mint;

#[derive(Accounts)]
#[instruction(wallet_limit_bump: u8, raffle_ticket_bump: u8, escrow_bump: u8)]
pub struct BuyRaffleTicket<'info> {
    config: Account<'info, Config>,
    #[account(
        mut,
        has_one = config,
        seeds = [PREFIX.as_bytes(), config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump,
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        seeds=[PREFIX.as_bytes(), LAUNCH_STAGES.as_bytes(), candy_machine.key().as_ref()],
        bump=launch_stages_info.bump,
        constraint = launch_stages_info.authority == candy_machine.authority
            && launch_stages_info.candy_machine == candy_machine.key()
            @ ErrorCode::InvalidLaunchStagesInfoFields
    )]
    launch_stages_info: Box<Account<'info, LaunchStagesInfo>>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [
            RAFFLE_TICKET.as_bytes(),
            candy_machine.key().as_ref(),
            payer.key().as_ref(),
        ],
        bump,
        space = RaffleTicket::SIZE,
    )]
    raffle_ticket: Account<'info, RaffleTicket>,
    #[account(mut)]
    pay_from: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Should be ATA, checked in code
    raffle_escrow: UncheckedAccount<'info>,
    payment_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            WALLET_LIMIT.as_bytes(),
            candy_machine.key().as_ref(),
            payer.key().as_ref()
        ],
        bump
    )]
    /// CHECK: checked in get_wallet_limit_info
    wallet_limit_info: UncheckedAccount<'info>,
    #[account(address = sysvar::slot_hashes::id())]
    /// CHECK: checked above, cannot afford to deserialize so it is unchecked
    slot_hashes: UncheckedAccount<'info>,
    notary: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

pub fn handle(
    ctx: Context<BuyRaffleTicket>,
    wallet_limit_bump: u8,
    raffle_ticket_bump: u8,
    escrow_bump: u8,
    curr_time: i64,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let launch_stages_info = &mut ctx.accounts.launch_stages_info;
    let wallet_limit_info = &mut ctx.accounts.wallet_limit_info;
    let raffle_ticket = &mut ctx.accounts.raffle_ticket;
    let payer = &ctx.accounts.payer;
    let pay_from = &ctx.accounts.pay_from;
    let raffle_escrow = &ctx.accounts.raffle_escrow;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let token_program = &ctx.accounts.token_program;
    let payment_mint = &ctx.accounts.payment_mint;

    let raffle_ticket_ai = raffle_ticket.to_account_info();
    let raffle_ticket_data = raffle_ticket_ai.try_borrow_data()?;
    if raffle_ticket_data[..8] != RaffleTicket::discriminator() && raffle_ticket_data[..8] != [0; 8]
    {
        return Err(ErrorCode::InvalidDiscriminator.into());
    }

    let wallet_limit_ai = wallet_limit_info.to_account_info();

    if let Some(notary) = candy_machine.notary {
        if *payer.key != candy_machine.authority {
            let notary_account = &ctx.accounts.notary;
            if notary != notary_account.key() {
                return Err(ErrorCode::NotaryPublicKeyInvalid.into());
            }
            let as_signer = Signer::try_from(notary_account)?;
            if !as_signer.is_signer {
                return Err(ErrorCode::NotarySignatureNotProvided.into());
            }
        }
    }

    let stage_index = get_current_stage_index(curr_time, &launch_stages_info.stages);

    let (
        ticket_price,
        wallet_limit,
        payment_mint_addr,
        mut wallet_limit_info_per_stage,
        unwrapped_stage_index,
    ) = if let Some(s) = stage_index {
        let curr_stage = &launch_stages_info.stages[s];

        let wallet_limit_info_per_stage = get_wallet_limit_info(
            ctx.program_id,
            &payer.to_account_info(),
            &wallet_limit_ai,
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.rent,
            s,
            &[&[
                WALLET_LIMIT.as_bytes(),
                candy_machine.key().as_ref(),
                payer.key().as_ref(),
                &[wallet_limit_bump],
            ]],
        )?;

        match curr_stage.stage_type {
            LaunchStageType::Raffle => {
                if let WalletLimitSpecification::FixedLimit { limit: wl } = curr_stage.wallet_limit
                {
                    (
                        curr_stage.price,
                        wl,
                        curr_stage.payment_mint,
                        wallet_limit_info_per_stage,
                        s,
                    )
                } else {
                    // should never happen
                    return Err(ErrorCode::RaffleRequiresLimit.into());
                }
            }
            _ => return Err(ErrorCode::CurrentStageMismatch.into()),
        }
    } else {
        return Err(ErrorCode::NoMatchingLaunchStage.into());
    };

    if wallet_limit_info.data.borrow()[..8] != WalletLimitInfoPerStage::discriminator()
        && wallet_limit_info.data.borrow()[..8] != [0; 8]
    {
        return Err(ErrorCode::InvalidDiscriminator.into());
    }

    assert_ata_address(
        &raffle_ticket.key(),
        &token_program.key(),
        &payment_mint_addr,
        escrow_bump,
        &associated_token_program.key(),
        &raffle_escrow.key(),
    )?;
    // create raffle escrow if needed
    if raffle_escrow.data_is_empty() {
        let create_associated_token_data = associated_token::Create {
            payer: payer.to_account_info(),
            associated_token: raffle_escrow.to_account_info(),
            authority: raffle_ticket.to_account_info(),
            mint: payment_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: token_program.to_account_info(),
        };
        let create_associated_token_ctx = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            create_associated_token_data,
        );
        associated_token::create(create_associated_token_ctx)?;
    }

    let raffle_escrow_parsed: spl_token::state::Account =
        assert_initialized(&raffle_escrow.to_account_info())?;
    if pay_from.mint != payment_mint_addr
        || raffle_escrow_parsed.mint != payment_mint_addr
        || payment_mint.key() != payment_mint_addr
    {
        return Err(ErrorCode::MintMismatch.into());
    }
    if raffle_escrow_parsed.owner != raffle_ticket.key() {
        return Err(ErrorCode::TokenOwnerMismatch.into());
    }

    // check if we have already bought more tickets than limit
    if wallet_limit_info_per_stage.redeemed[unwrapped_stage_index].redeemed_raffle_tickets
        >= wallet_limit
    {
        return Err(ErrorCode::WalletLimitExceeded.into());
    }

    if payment_mint_addr == native_mint::id()
        && pay_from.to_account_info().lamports() == ticket_price
    {
        let close_account_data = CloseAccount {
            authority: payer.to_account_info(),
            destination: raffle_escrow.to_account_info(),
            account: pay_from.to_account_info(),
        };
        let close_account_ctx =
            CpiContext::new(token_program.to_account_info(), close_account_data);
        close_account(close_account_ctx)?;
        spl_sync_native(
            token_program.to_account_info(),
            raffle_escrow.to_account_info(),
            &[],
        )?;
    } else {
        spl_token_transfer(
            TokenTransferParams {
                amount: ticket_price,
                authority: payer.to_account_info(),
                source: pay_from.to_account_info(),
                destination: raffle_escrow.to_account_info(),
                token_program: token_program.to_account_info(),
            },
            &[],
        )?;
    }

    let id = candy_machine.raffle_tickets_purchased as u32;

    raffle_ticket.raffle_payer = payer.key();
    raffle_ticket.escrow_bump = escrow_bump;
    raffle_ticket.ticket_bump = raffle_ticket_bump;
    raffle_ticket.candy_machine = candy_machine.key();
    candy_machine.raffle_tickets_purchased = candy_machine
        .raffle_tickets_purchased
        .checked_add(1)
        .ok_or(ErrorCode::NumericalOverflowError)?;

    raffle_ticket.ids.push(id);
    wallet_limit_info_per_stage.redeemed[unwrapped_stage_index].redeemed_raffle_tickets =
        wallet_limit_info_per_stage.redeemed[unwrapped_stage_index]
            .redeemed_raffle_tickets
            .checked_add(1)
            .ok_or(ErrorCode::NumericalOverflowError)?;

    // update random seed to add entropy
    let slot_hashes_data = ctx.accounts.slot_hashes.data.borrow();
    let most_recent_hashes = array_ref![slot_hashes_data, 4, 8];
    let hash_bytes = hashv(&[
        &candy_machine.raffle_seed.to_le_bytes(),
        most_recent_hashes,
        &curr_time.to_le_bytes(),
    ])
    .to_bytes();
    candy_machine.raffle_seed = u64::from_le_bytes(*array_ref![hash_bytes, 0, 8]);

    let mut wallet_limit_data = &mut wallet_limit_ai.data.borrow_mut()[8..];
    wallet_limit_info_per_stage.serialize(&mut wallet_limit_data)?;

    Ok(())
}
