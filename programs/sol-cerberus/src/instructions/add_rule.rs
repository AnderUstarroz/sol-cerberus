use crate::state::app::App;
use crate::state::rule::*;
use crate::utils::{valid_rules, utc_now};
use crate::Errors;
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 app_id (Pubkey)
// + 4 + 16 role (string)
// + 4 + 16 resource (string)
// + 20 permission (string)
// + 8 created_at (i64)
// + 1 + 8 expires_at Option<i64>
// + 1 bump
// total = 8 + 32 + 4 + 16 + 4 + 16 + 20 + 8 + 1 + 8 + 1 = 118
#[derive(Accounts)]
#[instruction(rule_data:RuleData)]
pub struct AddRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        init,
        payer = authority,
        space = 118,
        seeds = [rule_data.role.as_ref(), rule_data.resource.as_ref(), rule_data.permission.as_ref(), app.id.key().as_ref()], 
        constraint = valid_rules(&rule_data.role, &rule_data.resource, &rule_data.permission)  @ Errors::InvalidRule,
        bump
    )]
    pub rule: Account<'info, Rule>,
    pub system_program: Program<'info, System>,
}

pub fn add_rule(
    ctx: Context<AddRule>,
    rule_data:RuleData
) -> Result<()> {
    let rule = &mut ctx.accounts.rule;
    rule.bump = *ctx.bumps.get("rule").unwrap();
    rule.app_id = ctx.accounts.app.id;
    rule.role = rule_data.role;
    rule.resource = rule_data.resource;
    rule.permission = rule_data.permission;
    rule.expires_at = rule_data.expires_at;
    rule.created_at = utc_now();
    Ok(())
}
