use crate::state::app::App;
use crate::state::rule::*;
use crate::Errors;
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 app_id (Pubkey)
// + 1 bump
// + 20 role string
// + 20 resource string
// + 20 permission string
// total = 8 + 32  + 1 + 20 + 20 + 20 = 101
#[derive(Accounts)]
#[instruction(role: String, resource:String, permission:String)]
pub struct AddRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        init,
        payer = authority,
        space = 101,
        seeds = [role.as_ref(), resource.as_ref(), permission.as_ref(), app.id.key().as_ref()], 
        constraint = valid_rule(&role, &resource, &permission)  @ Errors::InvalidRule,
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
    rule.app_id = ctx.accounts.app.id;
    rule.bump = *ctx.bumps.get("rule").unwrap();
    rule.role = role;
    rule.resource = resource;
    rule.permission = permission;
    Ok(())
}
