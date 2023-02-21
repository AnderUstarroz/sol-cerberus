use crate::state::app::App;
use crate::state::role::Role;
use anchor_lang::prelude::*;

#[derive(Accounts)]
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
        close = collector,
        seeds = [role.role.as_ref(), role.address.key().as_ref(), app.id.key().as_ref()], 
        bump = role.bump,
    )]
    pub role: Account<'info, Role>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
}
