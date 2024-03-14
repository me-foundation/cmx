use crate::{
    constants::{MAX_CID_LENGTH, MAX_COLLECTION_LENGTH, MAX_GATEWAY_LENGTH},
    errors::ErrorCode,
    state::{Config, Creator},
};
use anchor_lang::{prelude::*, Accounts};
use mpl_token_metadata::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigArgs {
    pub gateway: Option<String>,
    pub cid: Option<String>,
    pub collection_name: Option<String>,
    pub symbol: Option<String>,
    pub seller_fee_basis_points: Option<u16>,
    pub creators: Option<Vec<Creator>>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, has_one = authority)]
    config: Account<'info, Config>,
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<UpdateConfig>, params: UpdateConfigArgs) -> Result<()> {
    let config = &mut ctx.accounts.config;

    let UpdateConfigArgs {
        gateway,
        cid,
        collection_name,
        symbol,
        seller_fee_basis_points,
        creators,
    } = params;

    if let Some(_gateway) = gateway {
        if _gateway.len() > MAX_GATEWAY_LENGTH {
            return Err(ErrorCode::GatewayTooLong.into());
        }
        config.gateway = _gateway;
    }
    if let Some(_cid) = cid {
        if _cid.len() > MAX_CID_LENGTH {
            return Err(ErrorCode::CIDLengthTooLong.into());
        }
        config.cid = _cid;
    }
    if let Some(_collection_name) = collection_name {
        if _collection_name.len() > MAX_COLLECTION_LENGTH {
            return Err(ErrorCode::CollectionNameTooLong.into());
        }
        config.collection_name = _collection_name;
    }
    if let Some(_symbol) = symbol {
        if _symbol.len() > MAX_SYMBOL_LENGTH {
            return Err(ErrorCode::SymbolTooLong.into());
        }
        config.symbol = _symbol;
    }
    if let Some(_seller_fee_basis_points) = seller_fee_basis_points {
        if _seller_fee_basis_points > 10000 {
            return Err(ErrorCode::SellerFeeBasisPointsOutOfRange.into());
        }
        config.seller_fee_basis_points = _seller_fee_basis_points;
    }
    if let Some(_creators) = creators {
        if _creators.len() > MAX_CREATOR_LIMIT {
            return Err(ErrorCode::TooManyCreators.into());
        }
        config.creators = _creators;
    }

    Ok(())
}
