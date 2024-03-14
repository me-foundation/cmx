use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Account does not have correct owner!")] // 0x1770
    IncorrectOwner,
    #[msg("Account is not initialized!")] // 0x1771
    Uninitialized,
    #[msg("Mint Mismatch!")] // 0x1772
    MintMismatch,
    #[msg("Index greater than length!")] // 0x1773
    IndexGreaterThanLength,
    #[msg("Config must have atleast one entry!")] // 0x1774
    ConfigMustHaveAtleastOneEntry,
    #[msg("Numerical overflow error!")] // 0x1775
    NumericalOverflowError,
    #[msg("Can only provide up to 4 creators to candy machine!")] // 0x1776
    TooManyCreators,
    #[msg("Uuid must be exactly of 6 length")] // 0x1777
    UuidMustBeExactly6Length,
    #[msg("Not enough tokens to pay for this minting")] // 0x1778
    NotEnoughTokens,
    #[msg("Not enough SOL to pay for this minting")] // 0x1779
    NotEnoughSOL,
    #[msg("Token transfer failed")] // 0x177a
    TokenTransferFailed,
    #[msg("Candy machine is empty!")] // 0x177b
    CandyMachineEmpty,
    #[msg("Candy machine is not live yet!")] // 0x177c
    CandyMachineNotLiveYet,
    #[msg("Number of config lines must be at least number of items available")] // 0x177d
    ConfigLineMismatch,
    #[msg("CID must be less than 255 characters")] // 0x177e
    CIDLengthTooLong,
    #[msg("Collection name must be less than 100 characters")] // 0x177f
    CollectionNameTooLong,
    #[msg("Notary signature not provided")] // 0x1780
    NotarySignatureNotProvided,
    #[msg("Invalid notary public key provided")] // 0x1781
    NotaryPublicKeyInvalid,
    #[msg("Wallet limit exceeded")] // 0x1782
    WalletLimitExceeded,
    #[msg("Derived key invalid")] // 0x1783
    DerivedKeyInvalid,
    #[msg("Too many launch stages (max 4)")] // 0x1784
    TooManyLaunchStages,
    #[msg("Invalid authority or candy_machine on launch_stages_info")] // 0x1785
    InvalidLaunchStagesInfoFields,
    #[msg("Symbol too long")] // 0x1786
    SymbolTooLong,
    #[msg("Current launch stage is invalid for stage required in the requested operation")]
    // 0x1787
    CurrentStageMismatch,
    #[msg("Invalid launch stage type")] // 0x1788
    InvalidLaunchStage,
    #[msg("Too many raffle related stages, only 1 is allowed")] // 0x1789
    TooManyRaffleStages,
    #[msg("Launch stage has start time after end time")] // 0x178a
    LaunchStageStartAfterEnd,
    #[msg("Launch stage list is not in order of earliest stage to latest stage")] // 0x178b
    LaunchStagesOutOfTimeOrder,
    #[msg("Cannot find matching stage for current time in launch stages")] // 0x178c
    NoMatchingLaunchStage,
    #[msg("Allowing too many tickets to be purchased")] // 0x178d
    TooManyRaffleTickets,
    #[msg("Raffle stage cannot be found")] // 0x178e
    RaffleNotFound,
    #[msg("Random seed used to settle auction does not hash into expected value")] // 0x178f
    RandomHashMismatch,
    #[msg("There are no ids in the given raffle ticket")] // 0x1790
    RaffleTicketEmpty,
    #[msg("Raffle ticket ID is not a winner")] // 0x1791
    RaffleTicketNotWinner,
    #[msg("Launch stages cannot be empty")] // 0x1792
    NoLaunchStages,
    #[msg("Raffle stage type requires a wallet limit that is fixed")] // 0x1793
    RaffleRequiresLimit,
    #[msg("No stage is active at this time")] // 0x1794
    StageNotActive,
    #[msg("Trying to update to too few items")] // 0x1795
    TooFewItemsAvailable,
    #[msg("Accounts assumed to be uninitialized are already in use")] // 0x1796
    AccountsAlreadyInUse,
    #[msg("Accounts expected to be initialized are not initialized")] // 0x1797
    AccountsUninitialized,
    #[msg("Edition account's key indicates that it is not an edition")] // 0x1798
    EditionKeyNotEdition,
    #[msg("Order account is not fully populated")] // 0x1799
    OrderAccountNotPopulated,
    #[msg("Attempted to mint index that was already minted due to shuffle")] // 0x179a
    CannotMintInOrderAfterRandom,
    #[msg("Mint using this candy machine has not finished")] // 0x179b
    MintNotFinished,
    #[msg("Token account owner is not what we expected")] // 0x179c
    TokenOwnerMismatch,
    #[msg("Authority has to mint for itself")] // 0x179d
    AuthorityHasToMintForSelf,
    #[msg("Discriminator different from what was expected")] // 0x179e
    InvalidDiscriminator,
    #[msg("User limit needs to have notary signature")] // 0x179f
    UserLimitNeedsNotary,
    #[msg("Given receiving token account(s) does not match expected")] // 0x17a0
    ReceivingTokenMismatch,
    #[msg("Raffle currently does not support supply per stage")] // 0x17a1
    RaffleDoesNotSupportSupply,
    #[msg("Given ATA address is not what is expected")] // 0x17a2
    AtaMismatch,
    #[msg("Given stage supply will not allow the mint to complete")] // 0x17a3
    InsufficientStageSupply,
    #[msg("Cannot delete stages")] // 0x17a4
    CannotDeleteStages,
    #[msg("Missing user limit")] // 0x17a5
    MissingUserLimit,
    #[msg("Stage supply has been exhausted")] // 0x17a6
    StageEmpty,
    #[msg("Stages do not have sufficient gap time between them")] // 0x17a7
    InsufficientStageGap,
    #[msg("Gateway too long")] // 0x17a8
    GatewayTooLong,
    #[msg("Seller fee basis points must be between 0 and 10000")] // 0x17a9
    SellerFeeBasisPointsOutOfRange,
    #[msg("Failed to deserialize wallet limit info")] // 0x17aa
    FailedToDeserializeWalletLimitInfo,
    #[msg("Cannot mint in lite mode after candy machine was used in normal mode")] // 0x17ab
    CannotMintInLiteMode,
    #[msg("Cannot mint in normal mode after candy machine was used in lite mode")] // 0x17ac
    CannotMintInNormalMode,
    #[msg("notary_required length must match stage length")] // 0x17ad
    NotaryRequiredLengthMismatch,
    #[msg("Variable limit not supported")] // 0x17ae
    VariableLimitNotSupported,
    #[msg("Non lite candy machine must have notary_required set to all true")] // 0x17af
    NonLiteCandyMachineInvalidNotaryRequired,
    #[msg("Cannot thaw due to expiry")] // 0x17b0
    CannotThawDueExpiry,
    #[msg("Invalid delegate")] // 0x17b1
    InvalidDelegate,
    #[msg("Cannot mint Open Editions")] // 0x17b2
    CannotMintOpenEditions,
}
