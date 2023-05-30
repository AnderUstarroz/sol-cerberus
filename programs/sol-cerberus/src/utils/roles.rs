use anchor_lang::prelude::*;

pub fn address_or_wildcard(address: &Option<Pubkey>) -> &[u8] {
    if address.is_none() {
        return b"*".as_ref();
    }
    address.as_ref().unwrap().as_ref()
}
