use crate::state::app::App;
use crate::state::assigned_role::AssignedRole;
use crate::utils::rules::*;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(role: String, address:Pubkey)]
pub struct DeleteAssignedRole<'info> {
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
        seeds = [b"role".as_ref(), role.as_ref(), address.key().as_ref()], 
        constraint = valid_rule(false, &role)  @ Errors::InvalidRole,
        bump = assigned_role.bump,
    )]
    pub assigned_role: Account<'info, AssignedRole>,
    /// CHECK: Destination of the funds
    #[account(mut)]
    destination: AccountInfo<'info>,
}
