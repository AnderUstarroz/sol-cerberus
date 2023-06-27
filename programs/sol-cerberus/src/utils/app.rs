use crate::App;
use crate::Errors;
use crate::FEE;
use crate::PROGRAM_AUTHORITY;
use anchor_lang::prelude::*;

pub fn allowed_authority(authority: &Pubkey, app_authority: &Pubkey) -> bool {
    return authority.key() == app_authority.key() || authority.key() == PROGRAM_AUTHORITY.key();
}

/// Gets the default fee applied to each "Allowed" request
pub fn get_fee(app: &App) -> u64 {
    if FEE.is_none() {
        return 0;
    }
    if app.fee.is_some() {
        return app.fee.unwrap();
    }
    FEE.unwrap()
}

/// Deducts the rent exemption price from the current fee, to prevent users from paying double.
pub fn subtract_rent_exemption_from_fee(fee: u64) -> u64 {
    match Rent::get() {
        Ok(rent) => {
            let rent_paid = rent.minimum_balance(9);
            if rent_paid > fee {
                0
            } else {
                fee - rent_paid
            }
        }
        Err(_) => {
            fee // Returns the default fee when Rent sysvar is not defined.
        }
    }
}

/// Only Program authority can modify the provided field
pub fn program_authority_field<T: PartialEq>(
    authority: &Pubkey,
    old_value: T,
    new_value: T,
) -> Result<T> {
    if old_value == new_value || authority.key() == PROGRAM_AUTHORITY.key() {
        return Ok(new_value);
    }
    err!(Errors::UnauthorizedProgramAuthority)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AccountTypes;
    use solana_program::pubkey;

    #[test]
    fn test_valid_authority() {
        // Generate a new random keypair and get the public key from the keypair.
        let valid_pubkey1: Pubkey = pubkey!("6kJuLfs8BrKwxy28FCmcPfp4d5stv4Sr6YgV15A6s7FK");
        let invalid_pubkey2: Pubkey = pubkey!("Ft9dAWwsFV8wFKmdgCJAe21ZnnqtXBBAdVB3cjUyRMY9");
        assert_eq!(allowed_authority(&valid_pubkey1, &valid_pubkey1), true); // Matching authorities are allowed
        assert_eq!(allowed_authority(&invalid_pubkey2, &valid_pubkey1), false); // Different authorities are not allowed
        assert_eq!(allowed_authority(&PROGRAM_AUTHORITY, &valid_pubkey1), true);
        // Program authority always allowed
    }
    #[test]
    fn test_get_fee() {
        let mut app = App {
            id: pubkey!("6kJuLfs8BrKwxy28FCmcPfp4d5stv4Sr6YgV15A6s7FK"),
            authority: pubkey!("6kJuLfs8BrKwxy28FCmcPfp4d5stv4Sr6YgV15A6s7FK"),
            recovery: None, // Only recovery or authority accounts can update the App Authority.
            bump: 0,
            name: "test".to_string(),
            roles_updated_at: 0,
            rules_updated_at: 0,
            cached: false,
            fee: None,
            account_type: AccountTypes::Basic as u8,
            expires_at: None,
        };
        assert_eq!(get_fee(&app), if FEE.is_some() { FEE.unwrap() } else { 0 });
        app.fee = Some(10);
        assert_eq!(get_fee(&app), 10);
    }

    #[test]
    fn test_program_authority_field() {
        let user = pubkey!("6kJuLfs8BrKwxy28FCmcPfp4d5stv4Sr6YgV15A6s7FK");
        assert_eq!(program_authority_field(&user, 1, 1), Ok(1));
        assert_eq!(
            program_authority_field(&user, 1, 2),
            err!(Errors::UnauthorizedProgramAuthority)
        );
        assert_eq!(program_authority_field(&PROGRAM_AUTHORITY, 1, 2), Ok(2));
    }
}
