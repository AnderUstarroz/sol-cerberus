use crate::state::app::App;
use crate::utils::app::allowed_authority;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = allowed_authority(&authority.key(), &app.authority)  @ Errors::Unauthorized,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
}
