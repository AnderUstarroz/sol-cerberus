use crate::PROGRAM_AUTHORITY;
use anchor_lang::prelude::*;

pub fn allowed_authority(authority: &Pubkey, app_authority: &Pubkey) -> bool {
    return authority.key() == app_authority.key() || authority.key() == PROGRAM_AUTHORITY.key();
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
