use crate::state::*;
use crate::utils::{
    assert_initialized, get_current_stage_index, get_next_nft_index, get_wallet_limit_info,
    spl_sync_native, spl_token_transfer, TokenTransferParams,
};
use crate::ErrorCode;
use crate::{
    constants::*,
    state::{CandyMachine, Config, Order},
};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::{self};
use anchor_lang::Discriminator;
use anchor_lang::{prelude::*, solana_program::sysvar, Accounts};
use anchor_spl::associated_token;
use anchor_spl::token::{close_account, CloseAccount};
use anchor_spl::{
    self,
    token::{Token, TokenAccount},
};
use mpl_token_metadata::instructions::{CreateCpiBuilder, MintCpiBuilder, UpdateCpiBuilder};
use mpl_token_metadata::types::{CreateArgs, MintArgs, PrintSupply, TokenStandard, UpdateArgs};
use spl_token::native_mint;

#[derive(Accounts)]
#[instruction(wallet_limit_bump: u8, in_order: bool, user_limit: Option<u8>)]
pub struct MintNFTMIP1<'info> {
    config: Box<Account<'info, Config>>,
    #[account(
        mut,
        has_one = config,
        seeds = [PREFIX.as_bytes(), config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump,
        constraint = candy_machine.mip1_ruleset.is_some(),
    )]
    candy_machine: Box<Account<'info, CandyMachine>>,
    /// CHECK: simply the receiver of the NFT, does not need to be checked, if the payer approves by signing, then it should be able to receive
    mint_receiver: UncheckedAccount<'info>,
    /// CHECK: constraint ensures this matches the candy machine authority
    #[account(mut, constraint = candy_machine_wallet_authority.key() == candy_machine.wallet_authority)]
    candy_machine_wallet_authority: UncheckedAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        mut,
        seeds=[PREFIX.as_bytes(), LAUNCH_STAGES.as_bytes(), candy_machine.key().as_ref()],
        bump=launch_stages_info.bump,
        constraint = launch_stages_info.authority == candy_machine.authority
            && launch_stages_info.candy_machine == candy_machine.key()
            @ ErrorCode::InvalidLaunchStagesInfoFields
    )]
    launch_stages_info: Box<Account<'info, LaunchStagesInfo>>,
    /// CHECK: checked manually, should be same as payer if stage payment is SOL, otherwise a valid token account
    #[account(mut)]
    pay_from: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = pay_to.owner == candy_machine.wallet_authority @ ErrorCode::TokenOwnerMismatch
    )]
    pay_to: Box<Account<'info, TokenAccount>>,
    /// CHECK: checked in line ~150
    notary: UncheckedAccount<'info>,
    // With the following accounts we aren't using anchor macros because they are CPI'd
    // through to token-metadata which will do all the validations we need on them.
    #[account(mut)]
    /// CHECK: checked when creating metadata by the token metadata program
    metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = mint.data_is_empty() @ ErrorCode::AccountsAlreadyInUse
    )]
    mint: Signer<'info>,
    #[account(
        mut,
        constraint = token_ata.data_is_empty() @ ErrorCode::AccountsAlreadyInUse
    )]
    /// CHECK: checked when the token ATA is created by CPI call
    token_ata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: checked when creating edition by the token metadata program
    master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: checked in CPI
    token_record: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            WALLET_LIMIT.as_bytes(),
            candy_machine.key().as_ref(),
            mint_receiver.key().as_ref()
        ],
        bump
    )]
    /// CHECK: checked in get_wallet_limit_info
    wallet_limit_info: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = candy_machine.order_info == order_info.key()
    )]
    order_info: AccountLoader<'info, Order>,
    #[account(address = sysvar::slot_hashes::id())]
    /// CHECK: checked above, do not want to deserialize the account so is left unchecked
    slot_hashes: UncheckedAccount<'info>,
    #[account(address = mpl_token_metadata::ID)]
    /// CHECK: checked above
    token_metadata_program: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    #[account(constraint = rule_set.key() == candy_machine.mip1_ruleset.unwrap_or_else(System::id))]
    /// CHECK: address is checked
    rule_set: UncheckedAccount<'info>,
    #[account(address = mpl_token_auth_rules::id())]
    /// CHECK: address is checked
    authorization_rules_program: UncheckedAccount<'info>,
    #[account(address = sysvar::instructions::id())]
    /// CHECK: check in cpi
    instructions: UncheckedAccount<'info>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle<'info>(
    ctx: Context<'_, '_, '_, 'info, MintNFTMIP1<'info>>,
    wallet_limit_bump: u8,
    in_order: bool,
    user_limit: Option<u8>,
    curr_time: i64,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let payer = &ctx.accounts.payer;
    let wallet_limit_info = &mut ctx.accounts.wallet_limit_info;
    let order_info = &mut ctx.accounts.order_info.load_mut()?;
    let config = &ctx.accounts.config;
    let launch_stages_info = &mut ctx.accounts.launch_stages_info;
    let mint = &ctx.accounts.mint;
    let token_ata = &ctx.accounts.token_ata;
    let rent = &ctx.accounts.rent;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let pay_from = &ctx.accounts.pay_from;
    let pay_to = &ctx.accounts.pay_to;
    let token_metadata_program = &ctx.accounts.token_metadata_program;

    if !candy_machine.is_open_edition.unwrap_or_default()
        && order_info.filled != (MAX_ITEMS_AVAILABLE as u32)
    {
        return Err(ErrorCode::OrderAccountNotPopulated.into());
    }

    let first_stage_time = launch_stages_info.stages[0].start_time;
    if curr_time < first_stage_time && *ctx.accounts.mint_receiver.key != candy_machine.authority {
        return Err(ErrorCode::CandyMachineNotLiveYet.into());
    }

    if *ctx.accounts.mint_receiver.key == candy_machine.authority
        && *payer.key != candy_machine.authority
    {
        return Err(ErrorCode::AuthorityHasToMintForSelf.into());
    }

    let total_redeemed = candy_machine
        .items_redeemed_normal
        .checked_add(candy_machine.raffle_tickets_purchased)
        .ok_or(ErrorCode::NumericalOverflowError)?;

    if candy_machine.is_lite {
        return Err(ErrorCode::CannotMintInNormalMode.into());
    }

    if !candy_machine.is_open_edition.unwrap_or_default()
        && total_redeemed >= candy_machine.items_available
    {
        return Err(ErrorCode::CandyMachineEmpty.into());
    }

    let stage_index = get_current_stage_index(curr_time, &launch_stages_info.stages);

    let (
        wallet_limit,
        price,
        payment_mint,
        payment_ata,
        mut wallet_limit_info_per_stage,
        unwrapped_stage_index,
    ) = if let Some(stage) = stage_index {
        let curr_stage = &launch_stages_info.stages[stage];
        let temp = match curr_stage.stage_type {
            LaunchStageType::NormalSale => (
                // user limit will always override wallet limit
                match curr_stage.wallet_limit {
                    WalletLimitSpecification::FixedLimit { limit: l } => {
                        if user_limit.is_some() {
                            // Some(std::cmp::min(ul, l))
                            user_limit
                        } else {
                            Some(l)
                        }
                    }
                    WalletLimitSpecification::VariableLimit => {
                        if user_limit.is_some() {
                            user_limit
                        } else {
                            return Err(ErrorCode::MissingUserLimit.into());
                        }
                    }
                    WalletLimitSpecification::NoLimit => user_limit,
                },
                curr_stage.price,
            ),
            _ => {
                return Err(ErrorCode::CurrentStageMismatch.into());
            }
        };
        let wallet_limit_info_per_stage = get_wallet_limit_info(
            ctx.program_id,
            &payer.to_account_info(),
            &wallet_limit_info.to_account_info(),
            &system_program.to_account_info(),
            rent,
            stage,
            &[&[
                WALLET_LIMIT.as_bytes(),
                candy_machine.key().as_ref(),
                ctx.accounts.mint_receiver.key().as_ref(),
                &[wallet_limit_bump],
            ]],
        )?;
        (
            temp.0,
            temp.1,
            curr_stage.payment_mint,
            curr_stage.payment_ata,
            wallet_limit_info_per_stage,
            stage,
        )
    } else if *ctx.accounts.mint_receiver.key != candy_machine.authority {
        return Err(ErrorCode::StageNotActive.into());
    } else {
        // we know that reciever is going to be authority, so it is okay
        let wallet_limit_info_per_stage = get_wallet_limit_info(
            ctx.program_id,
            &payer.to_account_info(),
            &wallet_limit_info.to_account_info(),
            &system_program.to_account_info(),
            rent,
            0,
            &[&[
                WALLET_LIMIT.as_bytes(),
                candy_machine.key().as_ref(),
                ctx.accounts.mint_receiver.key().as_ref(),
                &[wallet_limit_bump],
            ]],
        )?;
        (
            None,
            0,
            Pubkey::default(),
            Pubkey::default(),
            wallet_limit_info_per_stage,
            0_usize,
        )
    };

    if wallet_limit_info.data.borrow()[..8] != WalletLimitInfoPerStage::discriminator()
        && wallet_limit_info.data.borrow()[..8] != [0; 8]
    {
        return Err(ErrorCode::InvalidDiscriminator.into());
    }

    // check for notary signature if receiver is not authority or if a custom user_limit is set
    if let Some(notary) = candy_machine.notary {
        if *ctx.accounts.mint_receiver.key != candy_machine.authority
            && *ctx.accounts.payer.key != candy_machine.authority
        {
            // we don't check notary if authority signs in any way
            let notary_account = &ctx.accounts.notary;
            if notary != notary_account.key() {
                return Err(ErrorCode::NotaryPublicKeyInvalid.into());
            }
            let as_signer = Signer::try_from(notary_account)?;
            if !as_signer.is_signer {
                return Err(ErrorCode::NotarySignatureNotProvided.into());
            }
        }
    } else if user_limit.is_some() {
        return Err(ErrorCode::UserLimitNeedsNotary.into());
    }

    if let Some(si) = stage_index {
        let new_overflow = {
            let curr_stage = &launch_stages_info.stages[si];
            if si > 0 && curr_stage.minted_during_stage == 0 {
                // first time minting in this stage, need to set up overflow
                let prev_stage = &launch_stages_info.stages[si - 1];
                match prev_stage.stage_supply {
                    Some(sup) => {
                        let prev_supply = sup
                            .checked_add(prev_stage.previous_stage_unminted_supply)
                            .ok_or(ErrorCode::NumericalOverflowError)?;
                        let ret = prev_supply
                            .checked_sub(prev_stage.minted_during_stage)
                            .ok_or(ErrorCode::NumericalOverflowError)?;
                        Some(ret)
                    }
                    _ => {
                        // we do not allow overflow from stages with no set supply, so overflow = 0
                        Some(0)
                    }
                }
            } else {
                None
            }
        };

        let curr_stage_mut = &mut launch_stages_info.stages[si];
        if let Some(ov) = new_overflow {
            curr_stage_mut.previous_stage_unminted_supply = ov;
        }

        if *ctx.accounts.mint_receiver.key != candy_machine.authority {
            if let Some(supply) = curr_stage_mut.stage_supply {
                if curr_stage_mut.minted_during_stage
                    >= supply
                        .checked_add(curr_stage_mut.previous_stage_unminted_supply)
                        .ok_or(ErrorCode::NumericalOverflowError)?
                {
                    return Err(ErrorCode::StageEmpty.into());
                }
            }
            curr_stage_mut.minted_during_stage = curr_stage_mut
                .minted_during_stage
                .checked_add(1)
                .ok_or(ErrorCode::NumericalOverflowError)?;
        }
    }

    // if wallet_limit for stage is not set, do not enforce any wallet limit
    if let Some(wl) = wallet_limit {
        msg!(
            "Wallet has redeemed {:?} nfts, limit is {:?}",
            wallet_limit_info_per_stage.redeemed[unwrapped_stage_index].redeemed_normal,
            wl
        );
        if *ctx.accounts.mint_receiver.key != candy_machine.authority
            && wallet_limit_info_per_stage.redeemed[unwrapped_stage_index].redeemed_normal >= wl
        {
            return Err(ErrorCode::WalletLimitExceeded.into());
        }
    }

    // Bump # redeemed if not authority
    if *ctx.accounts.mint_receiver.key != candy_machine.authority {
        wallet_limit_info_per_stage.redeemed[unwrapped_stage_index].redeemed_normal =
            wallet_limit_info_per_stage.redeemed[unwrapped_stage_index]
                .redeemed_normal
                .checked_add(1)
                .ok_or(ErrorCode::NumericalOverflowError)?;
    }

    // Authority doesn't have to pay if it is receiving the token
    // or if it is minting the token for someone else
    if *ctx.accounts.mint_receiver.key != candy_machine.authority
        && *payer.key != candy_machine.authority
    {
        if pay_to.key() != payment_ata {
            return Err(ErrorCode::AtaMismatch.into());
        }
        if payment_mint == native_mint::id() {
            invoke(
                &system_instruction::transfer(&payer.key(), &pay_to.key(), price),
                &[payer.to_account_info(), pay_to.to_account_info()],
            )?;
            spl_sync_native(
                token_program.to_account_info(),
                pay_to.to_account_info(),
                &[],
            )?;
        } else if price > 0u64 {
            let pay_from_token_account: spl_token::state::Account =
                assert_initialized(&pay_from.to_account_info())?;

            if pay_from_token_account.mint != payment_mint || pay_to.mint != payment_mint {
                return Err(ErrorCode::MintMismatch.into());
            }

            spl_token_transfer(
                TokenTransferParams {
                    amount: price,
                    authority: payer.to_account_info(),
                    source: pay_from.to_account_info(),
                    destination: pay_to.to_account_info(),
                    token_program: token_program.to_account_info(),
                },
                &[],
            )?;

            if pay_from_token_account.amount == 0u64
                && pay_from.to_account_info().lamports() != 0u64
                && payer.key() != candy_machine.authority
                && payer.key() == pay_from_token_account.owner
            {
                let close_account_data = CloseAccount {
                    authority: payer.to_account_info(),
                    destination: payer.to_account_info(),
                    account: pay_from.to_account_info(),
                };
                let close_account_ctx =
                    CpiContext::new(token_program.to_account_info(), close_account_data);
                close_account(close_account_ctx)?;
            }
        }
    }

    let authority_key = candy_machine.authority;
    let next_token_id = if candy_machine.is_open_edition.unwrap_or_default() {
        0
    } else {
        get_next_nft_index(
            candy_machine,
            order_info,
            &ctx.accounts.slot_hashes.to_account_info(),
            &curr_time,
            ctx.accounts.payer.key() == authority_key,
            in_order,
        )?
    };

    let uri = format!(
        "https://{:}.{:}/{:}.json",
        config.cid.trim_matches(char::from(0)),
        config.gateway.trim_matches(char::from(0)),
        next_token_id
    );

    candy_machine.items_redeemed_normal = candy_machine
        .items_redeemed_normal
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

    let nft_name = if candy_machine.is_open_edition.unwrap_or_default() {
        config
            .collection_name
            .trim_matches(char::from(0))
            .to_string()
    } else {
        format!(
            "{:} #{:}",
            config.collection_name.trim_matches(char::from(0)),
            next_token_id
        )
    };

    // Create mint, metadata, and master edition.
    let create_args = CreateArgs::V1 {
        name: nft_name,
        symbol: config.symbol.clone(),
        uri,
        seller_fee_basis_points: config.seller_fee_basis_points,
        creators: Some(creators),
        primary_sale_happened: true,
        is_mutable: true,
        token_standard: TokenStandard::ProgrammableNonFungible,
        collection: None,
        uses: None,
        collection_details: None,
        rule_set: candy_machine.mip1_ruleset,
        decimals: Some(0),
        print_supply: Some(PrintSupply::Zero),
    };

    let mut create_builder = CreateCpiBuilder::new(token_metadata_program);
    create_builder
        .metadata(&ctx.accounts.metadata)
        .master_edition(Some(&ctx.accounts.master_edition))
        .mint(&mint.to_account_info(), true)
        .authority(&payer.to_account_info())
        .payer(&payer.to_account_info())
        .update_authority(&candy_machine.to_account_info(), true)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .create_args(create_args)
        .invoke_signed(&[&authority_seeds])?;

    // Create ATA and mint token to it.
    let mut mint_builder = MintCpiBuilder::new(token_metadata_program);
    mint_builder
        .token(&token_ata.to_account_info())
        .token_owner(Some(&ctx.accounts.mint_receiver))
        .mint(mint)
        .metadata(&ctx.accounts.metadata)
        .master_edition(Some(&ctx.accounts.master_edition))
        .token_record(Some(&ctx.accounts.token_record))
        .authority(&candy_machine.to_account_info())
        .payer(&ctx.accounts.payer)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .spl_ata_program(&ctx.accounts.associated_token_program)
        .authorization_rules(match candy_machine.mip1_ruleset {
            Some(_) => Some(&ctx.accounts.rule_set),
            None => None,
        })
        .authorization_rules_program(match candy_machine.mip1_ruleset {
            Some(_) => Some(&ctx.accounts.authorization_rules_program),
            None => None,
        })
        .mint_args(MintArgs::V1 {
            amount: 1,
            authorization_data: None,
        })
        .invoke_signed(&[&authority_seeds])?;

    // Have to use the new update handler for pNFTs.
    let mut update_args = UpdateArgs::default();
    if let UpdateArgs::V1 {
        ref mut new_update_authority,
        ref mut primary_sale_happened,
        ..
    } = update_args
    {
        *new_update_authority = Some(candy_machine.authority);
        *primary_sale_happened = Some(true);
    } else {
        panic!("UpdateArgs should be V1");
    };

    msg!("update args: {:?}", update_args);

    let mut update_builder = UpdateCpiBuilder::new(token_metadata_program);
    update_builder
        .authority(&candy_machine.to_account_info())
        .token(Some(&token_ata.to_account_info()))
        .mint(mint)
        .metadata(&ctx.accounts.metadata)
        .edition(Some(&ctx.accounts.master_edition))
        .payer(&ctx.accounts.payer)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.instructions)
        .authorization_rules(match candy_machine.mip1_ruleset {
            Some(_) => Some(&ctx.accounts.rule_set),
            None => None,
        })
        .authorization_rules_program(match candy_machine.mip1_ruleset {
            Some(_) => Some(&ctx.accounts.authorization_rules_program),
            None => None,
        })
        .update_args(update_args)
        .invoke_signed(&[&authority_seeds])?;

    let mut wallet_limit_data = &mut wallet_limit_info.data.borrow_mut()[8..];
    wallet_limit_info_per_stage.serialize(&mut wallet_limit_data)?;

    Ok(())
}
