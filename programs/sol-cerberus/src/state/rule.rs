use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct RuleData {
    pub role: String,
    pub resource: String,
    pub permission: String,
    pub expires_at: Option<i64>,
}

#[account]
pub struct Rule {
    pub app_id: Pubkey,
    pub role: String,
    pub resource: String,
    pub permission: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub bump: u8,
}
