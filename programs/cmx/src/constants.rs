pub const MAX_NAME_LENGTH: usize = 32;
pub const MAX_GATEWAY_LENGTH: usize = 20;
pub const MAX_CID_LENGTH: usize = 255;
pub const UUID_LENGTH: usize = 6;
pub const MAX_COLLECTION_LENGTH: usize = MAX_NAME_LENGTH - 7; // format: "<name> #<u16>", u16 is <= 5 digits
pub const MAX_LAUNCH_STAGES: usize = 10;
pub const MAX_RAFFLE_TICKETS: usize = 10;
pub const MAX_ITEMS_AVAILABLE: usize = 50_000;
pub const MIN_STAGE_GAP_SECONDS: i64 = 60;

pub const PREFIX: &str = "candy_machine";
pub const WALLET_LIMIT: &str = "wallet_limit";
pub const LAUNCH_STAGES: &str = "launch_stages";
pub const RAFFLE_TICKET: &str = "raffle_ticket";
pub const ESCROW: &str = "escrow";
pub const FREEZE_STATE: &str = "freeze_state";

pub const BOT_TAX_AMOUNT_LAMPORTS: u64 = 1000000; // 0.001 SOL
