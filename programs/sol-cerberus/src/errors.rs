use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Only current Authority or Recovery accounts can update the App authority")]
    UnauthorizedAuthorityUpdate,
    #[msg("Role, Resource or Permission must be betwen 1 and 16 alphanumeric characters long")]
    InvalidRule,
    #[msg("Role must be between 1 and 16 alphanumeric characters long")]
    InvalidRole,
    #[msg("The provided string is too short")]
    StringTooShort,
    #[msg("The provided string is too long")]
    StringTooLong,
    #[msg("The user does not have enough privileges to perform this action")]
    Unauthorized,
    #[msg("The Sol Cerberus APP ID does not match the one defined in the program")]
    InvalidAppID,
    #[msg("Invalid address type, mus be either 'Wallet', 'Nft', 'Collection' or a wildcard '*'")]
    InvalidAddressType,
    #[msg("Invalid namespace, must be either an u8 number (0-255) or a wildcard '*'")]
    InvalidNamespace,
    #[msg("SOL_CERBERUS_APP_ID is missing on lib.rs")]
    MissingSolCerberusAppId,
    #[msg("The Sol Cerberus Seed account is missing")]
    MissingSeedAccount,
    #[msg("Only program authority can perform this action")]
    UnauthorizedProgramAuthority,
    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,
}
