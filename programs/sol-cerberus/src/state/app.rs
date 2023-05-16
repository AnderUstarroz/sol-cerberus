use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct AppData {
    pub id: Pubkey,
    pub recovery: Option<Pubkey>,
    pub name: String,
}

#[account]
pub struct App {
    pub id: Pubkey,
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>, // Only recovery or authority accounts can update the App Authority.
    pub bump: u8,
    pub name: String,
    pub updated_at: i64,
    pub cached: bool,
    pub fee: Option<u64>,
}
