use crate::state::app::{App, AppChanged};
use crate::utils::app::allowed_authority;
use crate::utils::utc_now;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = collector,
        constraint = allowed_authority(&authority.key(), &app.authority)  @ Errors::Unauthorized,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
}

pub fn delete_app(ctx: Context<DeleteApp>) -> Result<()> {
    emit!(AppChanged {
        time: utc_now(),
        app_id: ctx.accounts.app.id,
        authority: ctx.accounts.app.authority,
    });
    Ok(())
}
