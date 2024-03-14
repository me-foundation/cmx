use crate::{constants::PREFIX, errors::ErrorCode, state::CandyMachine};
use anchor_lang::{prelude::*, Accounts};
use mpl_token_metadata::{
    accounts::Metadata, instructions::RemoveCreatorVerificationCpiBuilder,
    types::Key as TokenMetadataTypeKey,
};

#[derive(Accounts)]
pub struct UnverifyNonMasterEdition<'info> {
    #[account(
        seeds = [PREFIX.as_bytes(), candy_machine.config.key().as_ref(), candy_machine.uuid.as_bytes()],
        bump = candy_machine.bump
    )]
    candy_machine: Account<'info, CandyMachine>,
    #[account(address = mpl_token_metadata::ID)]
    /// CHECK: checked above
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: cool
    edition: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: cool
    metadata: UncheckedAccount<'info>,
}

pub fn handle(ctx: Context<UnverifyNonMasterEdition>) -> Result<()> {
    let candy_machine = &ctx.accounts.candy_machine;
    let edition = &ctx.accounts.edition;
    let metadata = &ctx.accounts.metadata;
    let token_metadata_program = &ctx.accounts.token_metadata_program;

    let metadata_parsed =
        &mut Metadata::safe_deserialize(&metadata.to_account_info().data.borrow())?;
    let mint_key = metadata_parsed.mint;

    if edition.data_is_empty() || metadata.data_is_empty() {
        return Err(ErrorCode::Uninitialized.into());
    }

    let (found_key, _) = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            token_metadata_program.key().as_ref(),
            mint_key.as_ref(),
            "edition".as_bytes(),
        ],
        &token_metadata_program.key(),
    );

    if found_key != edition.key() {
        msg!("expected {:?} got {:?}", edition.key(), found_key);
        return Err(ErrorCode::DerivedKeyInvalid.into());
    }

    // we should be able to safely read data from master_edition now
    let key = edition.try_borrow_data()?[0];

    if key != TokenMetadataTypeKey::EditionV1 as u8 {
        return Err(ErrorCode::EditionKeyNotEdition.into());
    }

    RemoveCreatorVerificationCpiBuilder::new(&token_metadata_program.to_account_info())
        .metadata(&metadata.to_account_info())
        .creator(&candy_machine.to_account_info())
        .invoke_signed(&[&[
            PREFIX.as_bytes(),
            candy_machine.config.key().as_ref(),
            candy_machine.uuid.as_bytes(),
            &[candy_machine.bump],
        ]])?;

    Ok(())
}
