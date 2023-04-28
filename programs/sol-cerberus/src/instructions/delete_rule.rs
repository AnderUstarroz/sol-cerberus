use crate::state::app::App;
use crate::state::rule::*;
use anchor_lang::prelude::*;
use crate::utils::app::allowed_authority;
use crate::Errors;

#[derive(Accounts)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = allowed_authority(&authority.key(), &app.authority)  @ Errors::Unauthorized,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        mut,
        close = collector,
        seeds = [rule.namespace.to_le_bytes().as_ref(), rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), app.id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,

}
