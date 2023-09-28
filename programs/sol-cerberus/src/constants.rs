use anchor_lang::prelude::*;
use solana_program::{declare_id, pubkey};

pub const FEE: Option<u64> = Some(5000);
pub const PROGRAM_AUTHORITY: Pubkey = pubkey!("SCfVPLT34pep4pHfnMTzSyMZ2kLcxjKTGS2phuiApz5");

pub mod metadata_program {
    use super::*;
    declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
}
