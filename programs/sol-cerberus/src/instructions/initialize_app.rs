use crate::utils::utc_now;
use crate::{state::app::*, utils::validate_string_len};
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 id (Pubkey)
// + 32 authority (Pubkey)
// + 1 + 32 Option<backup> (Pubkey)
// + 4 + 16 name (string)
// + 8 updated_at
// + 1 cached
// + 1 + 8 Option<u64> updated_at
// + 1 class
// + 1 + 8 Option<i64> expires_at
// + 1 bump
// total = 8 + 32  + 32 + 1 + 32 + 4 + 16 + 8 + 1 + 1 + 8 + 1 + 1 + 8 + 1 = 154
#[derive(Accounts)]
#[instruction(app_data: AppData)]
pub struct InitializeApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 154,
        seeds = [b"app".as_ref(), app_data.id.key().as_ref()], 
        bump
    )]
    pub app: Box<Account<'info, App>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_app(ctx: Context<InitializeApp>, app_data: AppData) -> Result<()> {
    let app = &mut ctx.accounts.app;
    app.id = app_data.id;
    app.class = Classes::Trial as u8;
    app.authority = ctx.accounts.authority.key();
    app.recovery = app_data.recovery;
    app.name = validate_string_len(&app_data.name, 0, 16)?;
    app.fee = None;
    app.cached = app_data.cached;
    app.updated_at = utc_now();
    app.expires_at = None;
    app.bump = *ctx.bumps.get("app").unwrap();
    emit!(AppChanged {
        time: app.updated_at,
        app_id: ctx.accounts.app.id,
        authority: ctx.accounts.app.authority,
    });
    Ok(())
}
