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
       0 => Rule (Normal permissions)
       1 => AssignRole (White list of roles that can be assigned by certain role)
       2 => DeleteAssignRole (White list of roles that can be deleted by certain role)
       3 => AddRuleNSRole (White list of namespaces and roles that can be created by certain role)
       4 => AddRuleResourcePerm (White list of resources and permissions that can be created by certain role)
       5 => DeleteRuleNSRole (White list of namespaces and roles that can be deleted by certain role)
       6 => DeleteRuleResourcePerm (White list of resources and permissions that can be deleted by certain role)
*/
#[repr(u8)]
pub enum Namespaces {
    Rule = 0,
    AssignRole = 1,
    DeleteAssignRole = 2,
    AddRuleNSRole = 3,
    AddRuleResourcePerm = 4,
    DeleteRuleNSRole = 5,
    DeleteRuleResourcePerm = 6,
}

#[account]
pub struct Rule {
    pub app_id: Pubkey,
    pub namespace: u8,
    pub role: String,
    pub resource: String,
    pub permission: String,
    pub expires_at: Option<i64>,
    pub bump: u8,
}

#[event]
pub struct RulesChanged {
    pub time: i64,
    #[index]
    pub app_id: Pubkey,
}
