use crate::state::app::App;
use crate::state::rule::*;
use crate::utils::valid_rules;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(role: String, resource:String, permission:String)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        mut,
        close = destination,
        seeds = [role.as_ref(), resource.as_ref(), permission.as_ref(), app.id.key().as_ref()], 
        constraint = valid_rules(&role, &resource, &permission)  @ Errors::InvalidRule,
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
    /// CHECK: Destination of the funds
    #[account(mut)]
    destination: AccountInfo<'info>,

}
