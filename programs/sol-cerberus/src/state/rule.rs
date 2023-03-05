use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct RuleData {
    pub namespace: u8,
    pub role: String,
    pub resource: String,
    pub permission: String,
    pub expires_at: Option<i64>,
}

/*
   Namespaces:
       0 => Normal rule
       1 => System rule
*/
#[account]
pub struct Rule {
    pub app_id: Pubkey,
    pub namespace: u8,
    pub role: String,
    pub resource: String,
    pub permission: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub bump: u8,
}

#[event]
pub struct RulesChanged {
    pub time: i64,
    #[index]
    pub app_id: Pubkey,
}
