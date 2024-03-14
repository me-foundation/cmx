use {
    crate::constants::*,
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
    mpl_token_metadata::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH},
};

#[account]
#[derive(Default)]
pub struct CandyMachine {
    pub authority: Pubkey,
    pub wallet_authority: Pubkey,
    pub config: Pubkey,
    pub items_redeemed_normal: u64,
    pub items_redeemed_raffle: u64,
    pub raffle_tickets_purchased: u64,
    pub uuid: String,
    pub items_available: u64,
    pub raffle_seed: u64,
    pub bump: u8,
    pub notary: Option<Pubkey>,
    pub order_info: Pubkey,
    pub is_lite: bool,
    // This is only used for mint_nft_lite, ignored in mint_nft which always requires notary
    pub notary_required: Vec<bool>,
    pub mip1_ruleset: Option<Pubkey>,
    // This is only applicable for mint_nft_mip1. mint_nft_lite, mint_nft_ocp and mint_nft do not support open editions as they are not being used as of Aug 2023.
    pub is_open_edition: Option<bool>,
}

impl CandyMachine {
    pub const SIZE: usize = 8
        + 32 // authority
        + 32 // wallet
        + 32 // config
        + 8 // items_redeemed_normal
        + 8 // items_redeemed_raffle
        + 1 + 6 // uuid
        + 8 // items_available
        + 8 // raffle_seed
        + 1 // bump
        + 1 + 32 // notary
        + 32 // order_info
        + 1 // is_lite
        + 4 + MAX_LAUNCH_STAGES // notary_required
        + 33 // Optional mip1_ruleset
        + 2 // optional is_open_edition
        + 318; // padding
}

#[account]
#[derive(Default)]
pub struct WalletLimitInfo {
    pub redeemed_normal: u8,
    pub redeemed_raffle_tickets: u8,
}

impl WalletLimitInfo {
    pub const SIZE: usize = 8
        + 1 // redeemed_normal
        + 1; // redeemed_raffle_tickets
}

#[account]
#[derive(Default)]
pub struct FreezeState {
    pub expiry: i64,
}

impl FreezeState {
    pub const SIZE: usize = 8
        + 32 * 2 // Pubkey
        + 8 // i64
        + 100; // padding
}

#[account]
#[derive(Default)]
pub struct WalletLimitInfoPerStage {
    pub redeemed: [RedeemedDuringStage; MAX_LAUNCH_STAGES],
}

impl WalletLimitInfoPerStage {
    pub const SIZE: usize = 8
        + MAX_LAUNCH_STAGES * RedeemedDuringStage::SIZE // redeemed
        + 172; // padding
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize, Copy)]
pub struct RedeemedDuringStage {
    pub redeemed_normal: u8,
    pub redeemed_raffle_tickets: u8,
}

impl RedeemedDuringStage {
    pub const SIZE: usize = 1 // redeemed_normal
        + 1; // redeemed_raffle_tickets
}

#[account]
#[derive(Default)]
pub struct RaffleTicket {
    pub ids: Vec<u32>,
    pub candy_machine: Pubkey,
    pub ticket_bump: u8,
    pub escrow_bump: u8,
    pub raffle_payer: Pubkey,
}

impl RaffleTicket {
    pub const SIZE: usize = 8
        + 4 + 4 * MAX_RAFFLE_TICKETS // ids
        + 32 // cmid
        + 1 // ticket bump
        + 1 // escrow nump
        + 32; // raffle_payer
}

#[account]
pub struct LaunchStagesInfo {
    pub bump: u8,
    pub authority: Pubkey,
    pub candy_machine: Pubkey,
    pub stages: Vec<LaunchStage>,
}

impl LaunchStagesInfo {
    pub const SIZE: usize = 8
        + 1 // bump
        + 32 // authority pubkey
        + 32 // candy_machine pubkey
        + 4 + MAX_LAUNCH_STAGES * LaunchStage::SIZE // stages
        + 500; // padding
}

// note that start time is inclusive, end time is exclusive
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum LaunchStageType {
    Invalid,
    NormalSale,
    Raffle,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum WalletLimitSpecification {
    NoLimit,
    FixedLimit { limit: u8 },
    VariableLimit,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchStage {
    pub stage_type: LaunchStageType,
    pub start_time: i64,
    pub end_time: i64,
    pub wallet_limit: WalletLimitSpecification,
    pub price: u64,
    pub stage_supply: Option<u32>,
    pub previous_stage_unminted_supply: u32,
    pub minted_during_stage: u32,
    pub payment_mint: Pubkey,
    pub payment_ata: Pubkey,
}

impl LaunchStage {
    pub const SIZE: usize = 1 // stage_type
        + 8 // start_time
        + 8 // end_time
        + 2 // wallet_limit
        + 8 // price
        + 1 + 4 // stage_supply
        + 4 // previous_stage_supply_overflow
        + 4 // minted_during_stage
        + 32 // payment_mint
        + 32; // payment_ata
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchStageArgs {
    pub stage_type: LaunchStageType,
    pub start_time: i64,
    pub end_time: i64,
    pub wallet_limit: WalletLimitSpecification,
    pub price: u64,
    pub stage_supply: Option<u32>,
    pub payment_mint_index: u8,
    pub payment_mint_ata_bump: u8,
}

#[account]
#[derive(Default)]
pub struct Config {
    pub authority: Pubkey,
    pub gateway: String,
    pub cid: String,
    pub uuid: String,
    pub collection_name: String,
    pub symbol: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    pub creators: Vec<Creator>,
    pub is_mutable: bool,
    pub retain_authority: bool,
}

impl Config {
    pub const SIZE: usize = 8
        + 32 // authority
        + MAX_GATEWAY_LENGTH // gateway
        + MAX_CID_LENGTH // cid
        + UUID_LENGTH // uuid
        + MAX_COLLECTION_LENGTH // collection_name
        + MAX_SYMBOL_LENGTH // symbol
        + 4 // seller_fee_basis_points
        + 4 + Creator::SIZE * MAX_CREATOR_LIMIT // creators
        + 1 // is_mutable
        + 1; // retain_authority
}

#[account(zero_copy)]
pub struct Order {
    pub filled: u32,
    pub candy_machine: Pubkey,
    // need to explicitly have length here so anchor can parse IDL (can't use MAX_ITEMS_AVAILABLE)
    pub indices: [u32; 50_000],
}

impl Default for Order {
    fn default() -> Self {
        Order {
            filled: 0,
            candy_machine: Pubkey::default(),
            indices: [0u32; MAX_ITEMS_AVAILABLE],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

impl Creator {
    pub const SIZE: usize = 32 // address
        + 1 // verified
        + 1; // share
}

#[event]
pub struct MintEvent {
    pub candy_machine_id: Pubkey,
    pub items_redeemed: u64,
}

#[event]
pub struct RaffleWinEvent {
    pub candy_machine_id: Pubkey,
    pub items_redeemed: u64,
    pub winner: Pubkey,
}

#[event]
pub struct RaffleLossEvent {
    pub candy_machine_id: Pubkey,
    pub items_redeemed: u64,
}
