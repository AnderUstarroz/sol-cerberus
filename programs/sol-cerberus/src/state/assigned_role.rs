use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AddressType {
    Wallet,
    NFT,
    Collection,
}
#[account]
pub struct AssignedRole {
    pub app_id: Pubkey,
    pub address: Pubkey,
    pub role: String,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
    pub bump: u8,
}
