use crate::state::app::*;
use crate::utils::utc_now;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateCache<'info> {
    pub authority: Signer<'info>, // Only current Authority is allowed
    #[account(
        mut,
        has_one = authority,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

pub fn update_cache(ctx: Context<UpdateCache>) -> Result<()> {
    let app = &mut ctx.accounts.app;
    app.updated_at = utc_now();
    emit!(AppChanged {
        time: app.updated_at,
        app_id: ctx.accounts.app.id,
        authority: ctx.accounts.app.authority,
    });
    Ok(())
}
