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
}
