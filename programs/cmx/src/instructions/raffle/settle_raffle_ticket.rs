use crate::{
    constants::*,
    state::*,
    utils::{
        assert_ata_address, check_ata, get_next_nft_index, get_raffle, is_ticket_winner,
        spl_sync_native, spl_token_transfer, TokenTransferParams,
    },
    ErrorCode,
};
use anchor_lang::{
    prelude::*,
    solana_program::{
        program::invoke, program_pack::Pack, system_instruction::create_account, sysvar,
    },
    Accounts,
};
use anchor_spl::{
    self, associated_token,
    token::{
        close_account, initialize_mint, mint_to, CloseAccount, InitializeMint, Mint, MintTo, Token,
        TokenAccount,
    },
};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3InstructionArgs, UpdateMetadataAccountV2CpiBuilder,
    },
    types::DataV2,
};
use spl_token::native_mint;

#[derive(Accounts)]
pub struct SettleRaffleTicket<'info> {
    config: Box<Account<'info, Config>>,
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
        mut,
        seeds = [
            RAFFLE_TICKET.as_bytes(),
            candy_machine.key().as_ref(),
            raffle_payer.key().as_ref(),
        ],
        bump = raffle_ticket.ticket_bump,
        has_one = raffle_payer
    )]
    raffle_ticket: Box<Account<'info, RaffleTicket>>,
    /// ATA where SPL tokens are held in escrow, owned by raffle_ticket
    #[account(mut)]
    raffle_escrow: Box<Account<'info, TokenAccount>>,
    /// ATA owned by wallet authority that we pay to if we win
    #[account(
        mut,
        constraint = pay_to.owner == candy_machine.wallet_authority @ ErrorCode::TokenOwnerMismatch
    )]
    pay_to: Box<Account<'info, TokenAccount>>,
    /// CHECK: either temporary token account that we use to refund SOL
    /// or ATA owned by raffle_payer to refund SPL
    #[account(mut)]
    refund_to: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: checked when token ATA is created by ATA program
    token_ata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: checked by checking seeds in the raffle_ticket account
    raffle_payer: UncheckedAccount<'info>, // person who paid for raffle tickets
    #[account(
        mut,
        constraint = candy_machine.order_info == order_info.key()
    )]
    order_info: AccountLoader<'info, Order>,
    notary: Signer<'info>,
    // With the following accounts we aren't using anchor macros because they are CPI'd
    // through to token-metadata which will do all the validations we need on them.
    #[account(mut)]
    /// CHECK: cool
    metadata: UncheckedAccount<'info>,
    #[account(mut)]
    mint: Signer<'info>,
    update_authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: create_master_edition instruction will not work if address is invalid
    master_edition: UncheckedAccount<'info>,
    #[account(address = sysvar::slot_hashes::id())]
    /// CHECK: address checked above
    slot_hashes: UncheckedAccount<'info>,
    #[account(address = mpl_token_metadata::ID)]
    /// CHECK: address checked above
    token_metadata_program: UncheckedAccount<'info>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<SettleRaffleTicket>, curr_time: i64) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let order_info = &mut ctx.accounts.order_info.load_mut()?;
    let config = &ctx.accounts.config;
    let launch_stages_info = &ctx.accounts.launch_stages_info;
    let raffle_ticket = &mut ctx.accounts.raffle_ticket;
    let rent = &ctx.accounts.rent;
    let payer = &ctx.accounts.payer;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let token_ata = &ctx.accounts.token_ata;
    let raffle_escrow = &ctx.accounts.raffle_escrow;
    let pay_to = &mut ctx.accounts.pay_to;
    let refund_to = &mut ctx.accounts.refund_to;
    let raffle_payer = &mut ctx.accounts.raffle_payer;
    let token_metadata_program = &ctx.accounts.token_metadata_program;

    if !mint.data_is_empty() || !token_ata.data_is_empty() {
        return Err(ErrorCode::AccountsAlreadyInUse.into());
    }

    if let Some(notary) = candy_machine.notary {
        if *ctx.accounts.payer.key != candy_machine.authority {
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

    let raffle_stage = get_raffle(&launch_stages_info.stages)?;
    if curr_time < raffle_stage.end_time {
        // can't settle before raffle ends
        return Err(ErrorCode::CurrentStageMismatch.into());
    }
    let is_native = raffle_stage.payment_mint == native_mint::id();
    let raffle_price = raffle_stage.price;
    if pay_to.mint != raffle_stage.payment_mint || refund_to.mint != raffle_stage.payment_mint {
        return Err(ErrorCode::MintMismatch.into());
    }
    if pay_to.owner != candy_machine.wallet_authority
        || (is_native && refund_to.owner != payer.key())
        || (!is_native && refund_to.owner != raffle_ticket.raffle_payer)
    {
        // we let payer be owner of refund_to since we always close it when wrapping SOL
        return Err(ErrorCode::TokenOwnerMismatch.into());
    }
    if pay_to.key() != raffle_stage.payment_ata {
        return Err(ErrorCode::ReceivingTokenMismatch.into());
    }
    assert_ata_address(
        &raffle_ticket.key(),
        &token_program.key(),
        &raffle_stage.payment_mint,
        raffle_ticket.escrow_bump,
        &ctx.accounts.associated_token_program.key(),
        &raffle_escrow.key(),
    )?;

    msg!("Randomizing with seed {}!", candy_machine.raffle_seed);

    let id = match raffle_ticket.ids.pop() {
        Some(n) => n,
        None => {
            return Err(ErrorCode::RaffleTicketEmpty.into());
        }
    };

    let cm_key = candy_machine.key();
    let payer_key = payer.key();
    let mint_key = mint.key();
    let ticket_seeds = [
        RAFFLE_TICKET.as_bytes(),
        cm_key.as_ref(),
        raffle_payer.key.as_ref(),
        &[raffle_ticket.ticket_bump],
    ];

    if is_native {
        // we first transfer all lamports to raffle_ticket for temporary holding
        if raffle_ticket.ids.is_empty() {
            // close raffle escrow
            close_account(CpiContext::new_with_signer(
                token_program.to_account_info(),
                CloseAccount {
                    authority: raffle_ticket.to_account_info(),
                    destination: raffle_ticket.to_account_info(),
                    account: raffle_escrow.to_account_info(),
                },
                &[&ticket_seeds],
            ))?;
        } else {
            spl_token_transfer(
                TokenTransferParams {
                    source: raffle_escrow.to_account_info(),
                    destination: refund_to.to_account_info(),
                    amount: raffle_price,
                    authority: raffle_ticket.to_account_info(),
                    token_program: token_program.to_account_info(),
                },
                &[&ticket_seeds],
            )?;
        }

        let close_account_data = CloseAccount {
            authority: payer.to_account_info(),
            destination: raffle_ticket.to_account_info(),
            account: refund_to.to_account_info(),
        };
        let close_account_ctx =
            CpiContext::new(token_program.to_account_info(), close_account_data);
        close_account(close_account_ctx)?;
    }

    if is_ticket_winner(id, candy_machine)? {
        // we first create mint account - will fail if mint is already initialized
        let mint_needed_rent = rent.minimum_balance(Mint::LEN);
        invoke(
            &create_account(
                &payer_key,
                &mint_key,
                mint_needed_rent,
                Mint::LEN as u64,
                &ctx.accounts.token_program.key(),
            ),
            &[
                payer.to_account_info(),
                mint.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        // initialize mint, payer is authority and update_authority for the mint
        let initialize_mint_data = InitializeMint {
            mint: mint.to_account_info(),
            rent: rent.to_account_info(),
        };
        let initialize_mint_ctx =
            CpiContext::new(token_program.to_account_info(), initialize_mint_data);
        initialize_mint(initialize_mint_ctx, 0, &payer_key, Some(&payer_key))?;

        // create associated token account
        let create_associated_token_data = associated_token::Create {
            payer: payer.to_account_info(),
            associated_token: token_ata.to_account_info(),
            authority: raffle_payer.to_account_info(),
            mint: mint.to_account_info(),
            system_program: system_program.to_account_info(),
            token_program: token_program.to_account_info(),
        };
        let create_associated_token_ctx = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            create_associated_token_data,
        );
        associated_token::create(create_associated_token_ctx)?;

        if mint.data_is_empty() || token_ata.data_is_empty() {
            return Err(ErrorCode::AccountsUninitialized.into());
        }

        let token_ata_deserialized =
            spl_token::state::Account::unpack_from_slice(&token_ata.data.borrow())?;
        check_ata(&token_ata_deserialized, &mint.key(), &raffle_payer.key())?;

        // mint 1 token to ATA
        let mint_token_ctx = CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: token_ata.to_account_info(),
                authority: payer.to_account_info(),
            },
        );
        mint_to(mint_token_ctx, 1)?;

        // we have won, so transfer funds and mint nft
        if is_native {
            let raffle_ticket_init_lamports = raffle_ticket.to_account_info().lamports();
            let pay_to_init_lamports = pay_to.to_account_info().lamports();
            **raffle_ticket.to_account_info().lamports.borrow_mut() = raffle_ticket_init_lamports
                .checked_sub(raffle_price)
                .ok_or(ErrorCode::NumericalOverflowError)?;
            **pay_to.to_account_info().lamports.borrow_mut() = pay_to_init_lamports
                .checked_add(raffle_price)
                .ok_or(ErrorCode::NumericalOverflowError)?;
            // balance account is needed since solana does balance checks within CPI calls
            spl_sync_native(
                token_program.to_account_info(),
                pay_to.to_account_info(),
                &[
                    raffle_ticket.to_account_info(),
                    refund_to.to_account_info(),
                    raffle_escrow.to_account_info(),
                ],
            )?;
        } else {
            spl_token_transfer(
                TokenTransferParams {
                    source: raffle_escrow.to_account_info(),
                    destination: pay_to.to_account_info(),
                    amount: raffle_price,
                    authority: raffle_ticket.to_account_info(),
                    token_program: token_program.to_account_info(),
                },
                &[&ticket_seeds],
            )?;
        }

        let next_token_id = get_next_nft_index(
            candy_machine,
            order_info,
            &ctx.accounts.slot_hashes.to_account_info(),
            &curr_time,
            false,
            true,
        )?;
        let uri = format!(
            "https://{:}.{:}/{:}.json",
            config.cid.trim_matches(char::from(0)),
            config.gateway.trim_matches(char::from(0)),
            next_token_id
        );

        candy_machine.items_redeemed_raffle = candy_machine
            .items_redeemed_raffle
            .checked_add(1)
            .ok_or(ErrorCode::NumericalOverflowError)?;

        let config_key = config.key();
        let authority_seeds = [
            PREFIX.as_bytes(),
            config_key.as_ref(),
            candy_machine.uuid.as_bytes(),
            &[candy_machine.bump],
        ];

        let mut creators: Vec<mpl_token_metadata::types::Creator> =
            vec![mpl_token_metadata::types::Creator {
                address: candy_machine.key(),
                verified: true,
                share: 0,
            }];

        for c in &config.creators {
            creators.push(mpl_token_metadata::types::Creator {
                address: c.address,
                verified: false,
                share: c.share,
            });
        }

        let data = DataV2 {
            name: format!(
                "{:} #{:}",
                config.collection_name.trim_matches(char::from(0)),
                next_token_id
            ),
            symbol: config.symbol.clone(),
            uri,
            seller_fee_basis_points: config.seller_fee_basis_points,
            creators: Some(creators),
            collection: None,
            uses: None,
        };

        CreateMetadataAccountV3Cpi {
            __program: token_metadata_program,
            metadata: &ctx.accounts.metadata,
            mint,
            mint_authority: payer,
            payer,
            update_authority: (&candy_machine.to_account_info(), true),
            system_program: &ctx.accounts.system_program,
            rent: None,
            __args: CreateMetadataAccountV3InstructionArgs {
                data,
                is_mutable: config.is_mutable,
                collection_details: None,
            },
        }
        .invoke_signed(&[&authority_seeds])?;

        CreateMasterEditionV3Cpi {
            __program: token_metadata_program,
            edition: &ctx.accounts.master_edition,
            mint,
            update_authority: &candy_machine.to_account_info(),
            mint_authority: payer,
            payer,
            metadata: &ctx.accounts.metadata,
            token_program: &ctx.accounts.token_program,
            system_program: &ctx.accounts.system_program,
            rent: None,
            __args: CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        }
        .invoke_signed(&[&authority_seeds])?;

        let mut update_builder = UpdateMetadataAccountV2CpiBuilder::new(token_metadata_program);
        update_builder
            .metadata(&ctx.accounts.metadata)
            .update_authority(&candy_machine.to_account_info())
            .new_update_authority(candy_machine.authority)
            .primary_sale_happened(true)
            .invoke_signed(&[&authority_seeds])?;

        // emit!(RaffleWinEvent {
        //     candy_machine_id: candy_machine.key(),
        //     items_redeemed: candy_machine.items_redeemed_normal
        //         + candy_machine.items_redeemed_raffle,
        //     winner: payer_key
        // });
    } else {
        // emit!(RaffleLossEvent {
        //     candy_machine_id: candy_machine.key(),
        //     items_redeemed: candy_machine.items_redeemed_normal
        //         + candy_machine.items_redeemed_raffle,
        // });
        // we have lost, so refund
        if is_native {
            let raffle_ticket_init_lamports = raffle_ticket.to_account_info().lamports();
            let raffle_payer_init_lamports = raffle_payer.to_account_info().lamports();
            **raffle_ticket.to_account_info().lamports.borrow_mut() = raffle_ticket_init_lamports
                .checked_sub(raffle_price)
                .ok_or(ErrorCode::NumericalOverflowError)?;
            **raffle_payer.to_account_info().lamports.borrow_mut() = raffle_payer_init_lamports
                .checked_add(raffle_price)
                .ok_or(ErrorCode::NumericalOverflowError)?;
        } else {
            spl_token_transfer(
                TokenTransferParams {
                    source: raffle_escrow.to_account_info(),
                    destination: refund_to.to_account_info(),
                    amount: raffle_price,
                    authority: raffle_ticket.to_account_info(),
                    token_program: token_program.to_account_info(),
                },
                &[&ticket_seeds],
            )?;
        }
    }

    let raffle_ticket_ai = raffle_ticket.to_account_info();
    if is_native {
        // refund rent back to payer
        let payer_ai = payer.to_account_info();
        let init_lamports_ticket = raffle_ticket_ai.lamports();
        let init_lamports_payer = payer_ai.lamports();
        let token_account_rent = rent.minimum_balance(pay_to.to_account_info().data_len());
        **payer_ai.lamports.borrow_mut() = init_lamports_payer
            .checked_add(token_account_rent)
            .ok_or(ErrorCode::NumericalOverflowError)?;
        **raffle_ticket_ai.lamports.borrow_mut() = init_lamports_ticket
            .checked_sub(token_account_rent)
            .ok_or(ErrorCode::NumericalOverflowError)?;
    }

    if raffle_ticket.ids.is_empty() {
        let raffle_payer_ai = raffle_payer.to_account_info();

        // close raffle ticket
        let init_lamports_ticket = raffle_ticket_ai.lamports();
        let init_lamports_raffle_payer = raffle_payer_ai.lamports();
        **raffle_ticket_ai.lamports.borrow_mut() = 0;
        **raffle_payer_ai.lamports.borrow_mut() = init_lamports_raffle_payer
            .checked_add(init_lamports_ticket)
            .ok_or(ErrorCode::NumericalOverflowError)?;
    }

    Ok(())
}
