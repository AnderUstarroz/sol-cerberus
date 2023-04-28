use crate::state::app::App;
use crate::state::role::Role;
use anchor_lang::prelude::*;
use crate::utils::app::allowed_authority;
use crate::Errors;

#[derive(Accounts)]
pub struct DeleteAssignedRole<'info> {
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
        seeds = [role.role.as_ref(), role.address.key().as_ref(), app.id.key().as_ref()], 
        bump = role.bump,
    )]
    pub role: Account<'info, Role>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
}
