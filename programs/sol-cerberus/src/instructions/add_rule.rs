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
#[instruction(role: String, resource:String, permission:String)]
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
        seeds = [role.as_ref(), resource.as_ref(), permission.as_ref(), app.id.key().as_ref()], 
        constraint = valid_rules(&role, &resource, &permission)  @ Errors::InvalidRule,
        bump
    )]
    pub rule: Account<'info, Rule>,
    pub system_program: Program<'info, System>,
}

pub fn add_rule(
    ctx: Context<AddRule>,
    role: String,
    resource: String,
    permission: String,
) -> Result<()> {
    let rule = &mut ctx.accounts.rule;
    rule.bump = *ctx.bumps.get("rule").unwrap();
    rule.app_id = ctx.accounts.app.id;
    rule.role = role;
    rule.resource = resource;
    rule.permission = permission;
    rule.created_at = utc_now();
    Ok(())
}
