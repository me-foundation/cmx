use {
    crate::constants::*,
    crate::errors::ErrorCode,
    crate::state::{
        CandyMachine, LaunchStage, LaunchStageArgs, LaunchStageType, Order, WalletLimitInfo,
        WalletLimitInfoPerStage, WalletLimitSpecification,
    },
    anchor_lang::{
        prelude::{msg, Account, AccountInfo, Pubkey, Rent, Result, Sysvar},
        solana_program::{
            hash::hash,
            hash::hashv,
            instruction::AccountMeta,
            program::{invoke, invoke_signed},
            program_pack::{IsInitialized, Pack},
            system_instruction,
        },
        Discriminator,
    },
    arrayref::array_ref,
    core::cell::RefMut,
    spl_token::{instruction::sync_native, state::Mint},
    std::convert::TryInto,
};

pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if account.owner != owner {
        Err(ErrorCode::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
///TokenTransferParams
pub struct TokenTransferParams<'a> {
    /// source
    /// CHECK: this is safe since the transfer instruction will fail if source is not of the correct mint type or is owned by someone else
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: this is safe since the destination is checked in the context declaration
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: this is safe since the transfer will fail if authority is not signer
    pub authority: AccountInfo<'a>,
    /// token_program
    /// CHECK: this is checked in the context definition
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(
    params: TokenTransferParams<'_>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
    } = params;

    let result = if signer_seeds.is_empty() {
        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                source.key,
                destination.key,
                authority.key,
                &[],
                amount,
            )?,
            &[source, destination, authority, token_program],
        )
    } else {
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                source.key,
                destination.key,
                authority.key,
                &[],
                amount,
            )?,
            &[source, destination, authority, token_program],
            signer_seeds,
        )
    };

    result.map_err(|_| ErrorCode::TokenTransferFailed.into())
}

#[inline(always)]
pub fn spl_sync_native<'a>(
    token_program: AccountInfo<'a>,
    token_account: AccountInfo<'a>,
    balance_accounts: &[AccountInfo<'a>],
) -> Result<()> {
    let accounts = [&[token_account.clone()], balance_accounts].concat();
    let sync_native_inst = &mut sync_native(token_program.key, token_account.key)?;
    for acc in balance_accounts {
        sync_native_inst
            .accounts
            .push(AccountMeta::new(*acc.key, false));
    }
    msg!("Syncing native account");

    invoke(sync_native_inst, &accounts)?;
    Ok(())
}

// #[inline(always)]
// pub fn create_or_allocate_account_raw<'a>(
//     program_id: Pubkey,
//     new_account_info: &AccountInfo<'a>,
//     rent_sysvar_info: &AccountInfo<'a>,
//     system_program_info: &AccountInfo<'a>,
//     payer_info: &AccountInfo<'a>,
//     size: usize,
//     signer_seeds: &[&[u8]],
// ) -> Result<()> {
//     let rent = &Rent::from_account_info(rent_sysvar_info)?;
//     let required_lamports = rent
//         .minimum_balance(size)
//         .max(1)
//         .saturating_sub(new_account_info.lamports());

//     if required_lamports > 0 {
//         msg!("Transfer {} lamports to the new account", required_lamports);
//         invoke(
//             &system_instruction::transfer(payer_info.key, new_account_info.key, required_lamports),
//             &[
//                 payer_info.clone(),
//                 new_account_info.clone(),
//                 system_program_info.clone(),
//             ],
//         )?;
//     }

//     msg!("Allocate space for the account");
//     invoke_signed(
//         &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
//         &[new_account_info.clone(), system_program_info.clone()],
//         &[signer_seeds],
//     )?;

//     msg!("Assign the account to the owning program");
//     invoke_signed(
//         &system_instruction::assign(new_account_info.key, &program_id),
//         &[new_account_info.clone(), system_program_info.clone()],
//         &[signer_seeds],
//     )?;
//     msg!("Completed assignation!");

//     Ok(())
// }

#[inline(always)]
pub fn check_ata(
    ata: &spl_token::state::Account,
    expected_mint: &Pubkey,
    expected_owner: &Pubkey,
) -> Result<()> {
    if ata.mint != *expected_mint {
        return Err(ErrorCode::MintMismatch.into());
    }
    if ata.owner != *expected_owner {
        return Err(ErrorCode::TokenOwnerMismatch.into());
    }
    Ok(())
}

pub fn assert_derivation(program_id: &Pubkey, account: &AccountInfo, path: &[&[u8]]) -> Result<u8> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account.key {
        return Err(ErrorCode::DerivedKeyInvalid.into());
    }
    Ok(bump)
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn assert_stages(
    launch_stages: &[LaunchStageArgs],
    mint_accounts: &[AccountInfo],
    ata_accounts: &[AccountInfo],
    token_program_key: &Pubkey,
    associated_token_program_key: &Pubkey,
    wallet_authority: &Pubkey,
    items_available: u64,
    notary_required: &[bool],
) -> Result<()> {
    if launch_stages.is_empty() {
        return Err(ErrorCode::NoLaunchStages.into());
    }
    if launch_stages.len() > MAX_LAUNCH_STAGES {
        return Err(ErrorCode::TooManyLaunchStages.into());
    }
    if ata_accounts.len() != mint_accounts.len() {
        return Err(ErrorCode::ReceivingTokenMismatch.into());
    }

    if notary_required.len() != launch_stages.len() {
        return Err(ErrorCode::NotaryRequiredLengthMismatch.into());
    }

    let (mut normal_count, mut raffle_count): (u8, u8) = (0, 0);
    let mut last_end: i64 = -1;
    let mut stage_supply_running_total: u32 = 0;
    let mut seen_unlimited_supply = false;

    for (i, stage) in launch_stages.iter().enumerate() {
        let mint_ai = &mint_accounts[stage.payment_mint_index as usize];
        let ata_ai = &ata_accounts[stage.payment_mint_index as usize];

        if stage.start_time - last_end < MIN_STAGE_GAP_SECONDS {
            return Err(ErrorCode::InsufficientStageGap.into());
        }
        match stage.stage_type {
            LaunchStageType::Invalid => return Err(ErrorCode::InvalidLaunchStage.into()),
            LaunchStageType::NormalSale => {
                normal_count += 1;
                assert_stage_times(stage.start_time, stage.end_time, last_end)?;
                last_end = stage.end_time;

                if let WalletLimitSpecification::VariableLimit = stage.wallet_limit {
                    if !notary_required[i] {
                        return Err(ErrorCode::VariableLimitNotSupported.into());
                    }
                }
            }
            LaunchStageType::Raffle => {
                if let WalletLimitSpecification::FixedLimit { limit: wl } = stage.wallet_limit {
                    if wl as usize > MAX_RAFFLE_TICKETS {
                        return Err(ErrorCode::TooManyRaffleTickets.into());
                    }
                } else {
                    return Err(ErrorCode::RaffleRequiresLimit.into());
                }

                if stage.stage_supply.is_some() {
                    return Err(ErrorCode::RaffleDoesNotSupportSupply.into());
                }
                raffle_count += 1;
                assert_stage_times(stage.start_time, stage.end_time, last_end)?;
                last_end = stage.end_time;
            }
        }

        // make sure the mint in which we receive payment is initialized correctly
        if !(assert_initialized::<Mint>(mint_ai)?.is_initialized) {
            return Err(ErrorCode::AccountsUninitialized.into());
        }
        assert_owned_by(mint_ai, &spl_token::id())?;

        assert_ata_address(
            wallet_authority,
            token_program_key,
            mint_ai.key,
            stage.payment_mint_ata_bump,
            associated_token_program_key,
            ata_ai.key,
        )?;

        let token_account: spl_token::state::Account = assert_initialized(ata_ai)?;
        if token_account.owner != *wallet_authority {
            return Err(ErrorCode::TokenOwnerMismatch.into());
        }
        if token_account.mint != *mint_ai.key {
            return Err(ErrorCode::MintMismatch.into());
        }

        match stage.stage_supply {
            Some(s) => {
                stage_supply_running_total = stage_supply_running_total
                    .checked_add(s)
                    .ok_or(ErrorCode::NumericalOverflowError)?;
            }
            None => {
                seen_unlimited_supply = true;
            }
        }
    }
    if (stage_supply_running_total as u64) < items_available && !seen_unlimited_supply {
        return Err(ErrorCode::InsufficientStageSupply.into());
    }
    msg!(
        "We have {} normal sales, {} raffle sales",
        normal_count,
        raffle_count
    );

    // we only support one raffle currently
    if raffle_count > 1 {
        return Err(ErrorCode::TooManyRaffleStages.into());
    }
    Ok(())
}

#[inline(always)]
pub fn assert_ata_address(
    wallet_address: &Pubkey,
    token_program_address: &Pubkey,
    spl_token_mint_address: &Pubkey,
    bump: u8,
    associated_token_program_address: &Pubkey,
    ata_address: &Pubkey,
) -> Result<()> {
    let to_check = Pubkey::create_program_address(
        &[
            &wallet_address.to_bytes(),
            &token_program_address.to_bytes(),
            &spl_token_mint_address.to_bytes(),
            &[bump],
        ],
        associated_token_program_address,
    );

    match to_check {
        Ok(addr) => {
            if addr != *ata_address {
                return Err(ErrorCode::AtaMismatch.into());
            }
        }
        _ => {
            return Err(ErrorCode::AtaMismatch.into());
        }
    }
    Ok(())
}

fn assert_stage_times(start_time: i64, end_time: i64, last_end: i64) -> Result<()> {
    if start_time >= end_time {
        return Err(ErrorCode::LaunchStageStartAfterEnd.into());
    }

    if last_end > start_time {
        return Err(ErrorCode::LaunchStagesOutOfTimeOrder.into());
    }
    Ok(())
}

pub fn get_current_stage_index(curr_time: i64, launch_stages: &[LaunchStage]) -> Option<usize> {
    for (index, stage) in launch_stages.iter().enumerate() {
        if stage.start_time <= curr_time && stage.end_time > curr_time {
            return Some(index);
        }
    }

    None
}

pub fn get_raffle(launch_stages: &[LaunchStage]) -> Result<&LaunchStage> {
    for stage in launch_stages {
        if let LaunchStageType::Raffle = stage.stage_type {
            return Ok(stage);
        }
    }

    Err(ErrorCode::RaffleNotFound.into())
}

pub fn is_ticket_winner(id: u32, candy_machine: &CandyMachine) -> Result<bool> {
    let seed = candy_machine.raffle_seed;

    let raffle_tickets = candy_machine.raffle_tickets_purchased as u32;
    let num_buckets = candy_machine
        .items_available
        .checked_sub(candy_machine.items_redeemed_normal)
        .ok_or(ErrorCode::NumericalOverflowError)? as u32;
    let offset = seed
        .checked_rem(raffle_tickets as u64)
        .ok_or(ErrorCode::NumericalOverflowError)? as u32;
    let average_bucket_size = raffle_tickets
        .checked_div(num_buckets)
        .ok_or(ErrorCode::NumericalOverflowError)?;
    let larger_buckets = raffle_tickets
        .checked_rem(num_buckets)
        .ok_or(ErrorCode::NumericalOverflowError)?;
    let my_bucket_index = raffle_tickets
        .checked_sub(offset)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_add(id)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_rem(raffle_tickets)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_rem(num_buckets)
        .ok_or(ErrorCode::NumericalOverflowError)?;
    let my_bucket_size = if my_bucket_index < larger_buckets {
        average_bucket_size + 1
    } else {
        average_bucket_size
    };

    // don't really need to check for overflow here since hash should handle it
    let bucket_salted_seed = my_bucket_index as u64 + seed + 1u64;
    let bucket_hash = hash(format!("{bucket_salted_seed}").as_bytes()).to_bytes();
    let bucket_random_seed = u128::from_le_bytes(bucket_hash[..16].try_into().unwrap());

    let winning_index_in_bucket = bucket_random_seed
        .checked_rem(my_bucket_size as u128)
        .ok_or(ErrorCode::NumericalOverflowError)?;

    let check_calc = (winning_index_in_bucket)
        .checked_mul(num_buckets as u128)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_add(offset as u128)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_add(my_bucket_index as u128)
        .ok_or(ErrorCode::NumericalOverflowError)?
        .checked_rem(raffle_tickets as u128)
        .ok_or(ErrorCode::NumericalOverflowError)? as u32;

    Ok(check_calc == id)
}

#[inline(never)]
#[allow(clippy::manual_swap)]
pub fn get_next_nft_index(
    candy_machine: &mut CandyMachine,
    order_info: &mut RefMut<Order>,
    slot_hashes: &AccountInfo,
    curr_time: &i64,
    is_authority: bool,
    in_order: bool,
) -> Result<u32> {
    let slot_hashes_data = slot_hashes.data.borrow();
    let most_recent_hashes = array_ref![slot_hashes_data, 4, 8];

    let hash_bytes = hashv(&[most_recent_hashes, &curr_time.to_le_bytes()]).to_bytes();
    let seed = u64::from_le_bytes(*array_ref![hash_bytes, 0, 8]);

    let mut next_token_index = candy_machine
        .items_redeemed_normal
        .checked_add(candy_machine.items_redeemed_raffle)
        .ok_or(ErrorCode::NumericalOverflowError)? as u32;

    if is_authority && in_order {
        if next_token_index != order_info.indices[next_token_index as usize] {
            return Err(ErrorCode::CannotMintInOrderAfterRandom.into());
        }
    } else {
        // Shuffle if not authority or not in_order
        let random_index = seed % (candy_machine.items_available - next_token_index as u64)
            + next_token_index as u64;

        // swap value at random_index with value at next_token_index
        let temp = order_info.indices[random_index as usize];
        order_info.indices[random_index as usize] = order_info.indices[next_token_index as usize];
        order_info.indices[next_token_index as usize] = temp;
        next_token_index = temp;
    }
    Ok(next_token_index)
}

#[inline(always)]
pub fn get_wallet_limit_info<'a>(
    program_id: &Pubkey,
    payer_ai: &AccountInfo<'a>,
    wallet_limit_ai: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    rent: &Sysvar<Rent>,
    current_stage_index: usize,
    seeds: &[&[&[u8]]],
) -> Result<Account<'a, WalletLimitInfoPerStage>> {
    if wallet_limit_ai.data_is_empty() {
        // allocate new wallet limit info per stage account
        let lamports_needed = rent.minimum_balance(WalletLimitInfoPerStage::SIZE);
        invoke_signed(
            &system_instruction::create_account(
                payer_ai.key,
                wallet_limit_ai.key,
                lamports_needed,
                WalletLimitInfoPerStage::SIZE as u64,
                program_id,
            ),
            &[
                payer_ai.clone(),
                wallet_limit_ai.clone(),
                system_program.clone(),
            ],
            seeds,
        )?;

        wallet_limit_ai.data.borrow_mut()[..8]
            .copy_from_slice(&WalletLimitInfoPerStage::discriminator());
        return Account::<WalletLimitInfoPerStage>::try_from(wallet_limit_ai);
    }

    if wallet_limit_ai.data.borrow()[..8] == WalletLimitInfo::discriminator()
        && wallet_limit_ai.data_len() == WalletLimitInfo::SIZE
    {
        // current is old wallet limit, need to upgrade to new wallet limit
        let curr_limit = Account::<WalletLimitInfo>::try_from(wallet_limit_ai)?; // upgrade by realloc, save current limits
        let redeemed_normal = curr_limit.redeemed_normal;
        let redeemed_raffle = curr_limit.redeemed_raffle_tickets;
        // zero out current data
        wallet_limit_ai
            .data
            .borrow_mut()
            .copy_from_slice(&[0; WalletLimitInfo::SIZE]);

        // realloc, write discriminator
        wallet_limit_ai.realloc(WalletLimitInfoPerStage::SIZE, true)?;
        wallet_limit_ai.data.borrow_mut()[..8]
            .copy_from_slice(&WalletLimitInfoPerStage::discriminator());

        let lamports_needed = rent.minimum_balance(WalletLimitInfoPerStage::SIZE);
        invoke(
            &system_instruction::transfer(payer_ai.key, wallet_limit_ai.key, lamports_needed),
            &[
                payer_ai.clone(),
                wallet_limit_ai.clone(),
                system_program.clone(),
            ],
        )?;

        let mut new_limit = Account::<WalletLimitInfoPerStage>::try_from(wallet_limit_ai)?;
        new_limit.redeemed[current_stage_index].redeemed_normal = redeemed_normal;
        new_limit.redeemed[current_stage_index].redeemed_raffle_tickets = redeemed_raffle;
        return Ok(new_limit);
    }

    Account::<WalletLimitInfoPerStage>::try_from(wallet_limit_ai)
}
