use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Only current Authority or Recovery accounts can update the App authority")]
    UnauthorizedAuthorityUpdate,
    #[msg("Role, Resource and Permission can have a maximum of 16 alphanumeric characters each")]
    InvalidRule,
}
