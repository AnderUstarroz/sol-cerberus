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
    pub name: String,
    pub bump: u8,
}
