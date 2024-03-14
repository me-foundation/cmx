use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Config, Creator},
};
use anchor_lang::{prelude::*, Accounts};
use mpl_token_metadata::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = payer,
        space = Config::SIZE,
    )]
    config: Account<'info, Config>,
    /// CHECK: does not need to be checked, pubkey is assigned
    authority: UncheckedAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConfigArgs {
    pub gateway: String,
    pub cid: String,
    pub uuid: String,
    pub collection_name: String,
    pub symbol: String,
    pub seller_fee_basis_points: u16,
    pub creators: Vec<Creator>,
    pub is_mutable: bool,
    pub retain_authority: bool,
}

pub fn handle(ctx: Context<InitializeConfig>, args: InitializeConfigArgs) -> Result<()> {
    let config_info = &mut ctx.accounts.config;
    let InitializeConfigArgs {
        gateway,
        cid,
        uuid,
        creators,
        collection_name,
        retain_authority,
        is_mutable,
        symbol,
        seller_fee_basis_points,
    } = args;

    if uuid.len() != UUID_LENGTH {
        return Err(ErrorCode::UuidMustBeExactly6Length.into());
    }

    if cid.len() > MAX_CID_LENGTH {
        return Err(ErrorCode::CIDLengthTooLong.into());
    }
    if collection_name.len() > MAX_COLLECTION_LENGTH {
        return Err(ErrorCode::CollectionNameTooLong.into());
    }

    // - 1 because we are going to be a creator
    if creators.len() > MAX_CREATOR_LIMIT - 1 {
        return Err(ErrorCode::TooManyCreators.into());
    }

    if symbol.len() > MAX_SYMBOL_LENGTH {
        return Err(ErrorCode::SymbolTooLong.into());
    }

    config_info.authority = *ctx.accounts.authority.key;
    config_info.creators = creators;
    config_info.uuid = uuid;
    config_info.cid = cid;
    config_info.gateway = gateway;
    config_info.retain_authority = retain_authority;
    config_info.is_mutable = is_mutable;
    config_info.seller_fee_basis_points = seller_fee_basis_points;
    config_info.collection_name = collection_name;
    config_info.symbol = symbol;

    Ok(())
}
