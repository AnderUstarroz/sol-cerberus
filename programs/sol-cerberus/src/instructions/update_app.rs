use crate::state::app::*;
use crate::utils::{program_authority_field, utc_now, validate_string_len};
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateApp<'info> {
    pub signer: Signer<'info>, // Only current Authority or Recovery key can update the Authority
    #[account(
        mut,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
        constraint = app.authority == signer.key() || (app.recovery.is_some() && app.recovery.unwrap() == signer.key())   @ Errors::UnauthorizedAuthorityUpdate,
    )]
    pub app: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

pub fn update_app(ctx: Context<UpdateApp>, app_data: UpdateAppData) -> Result<()> {
    let app = &mut ctx.accounts.app;
    app.authority = app_data.authority;
    app.recovery = app_data.recovery;
    app.name = validate_string_len(&app_data.name, 0, 16)?;
    app.account_type =
        program_authority_field(&app_data.authority, app.account_type, app_data.account_type)?;
    app.fee = program_authority_field(&app_data.authority, app.fee, app_data.fee)?;
    app.cached = app_data.cached;
    app.expires_at =
        program_authority_field(&app_data.authority, app.expires_at, app_data.expires_at)?;
    emit!(AppChanged {
        time: utc_now(),
        app_id: ctx.accounts.app.id,
        authority: ctx.accounts.app.authority,
    });
    Ok(())
}
