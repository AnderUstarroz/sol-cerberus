use anchor_lang::prelude::*;

///  Classes:
///     0 => Trial  (Apps with default fees)
///     1 => Free   (Apps with no fees)
#[repr(u8)]
pub enum Classes {
    Trial = 0,
    Free = 1,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct AppData {
    pub id: Pubkey,
    pub recovery: Option<Pubkey>,
    pub name: String,
    pub cached: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct UpdateAppData {
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>,
    pub name: String,
    pub cached: bool,
    pub fee: Option<u64>,
    pub class: u8,
    pub expires_at: Option<i64>,
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
    pub class: u8,
    pub expires_at: Option<i64>,
}

#[event]
pub struct AppChanged {
    pub time: i64,
    #[index]
    pub app_id: Pubkey,
    pub authority: Pubkey,
}

#[account]
pub struct Seed {
    pub initialized: bool,
}
