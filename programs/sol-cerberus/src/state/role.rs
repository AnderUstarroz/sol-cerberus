use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AddressType {
    Wallet,
    NFT,
    Collection,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AssignRoleData {
    pub address: Pubkey,
    pub role: String,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
}

#[account]
pub struct Role {
    pub app_id: Pubkey,
    pub address: Pubkey,
    pub role: String,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
    pub bump: u8,
}
