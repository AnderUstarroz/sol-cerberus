use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct AppData {
    pub id: Pubkey,
    pub recovery: Option<Pubkey>,
}

#[account]
pub struct App {
    pub id: Pubkey,
    pub authority: Pubkey,
    pub bump: u8,
    pub recovery: Option<Pubkey>, // Only recovery or authority accounts can update the App Authority.
}
