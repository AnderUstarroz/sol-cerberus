use crate::state::app::*;
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 id (Pubkey)
// + 32 authority (Pubkey)
// + 1 bump
// + 1 + 32 Option<backup> (Pubkey)
// total = 8 + 32  + 32 + 1 + 1 + 32 = 106
#[derive(Accounts)]
#[instruction(app_data: AppData)]
pub struct InitializeApp<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 106,
        seeds = [b"app".as_ref(), app_data.id.key().as_ref()], 
        bump
    )]
    pub app: Box<Account<'info, App>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_app(ctx: Context<InitializeApp>, app_data: AppData) -> Result<()> {
    let app = &mut ctx.accounts.app;
    app.id = app_data.id;
    app.authority = ctx.accounts.authority.key();
    app.bump = *ctx.bumps.get("app").unwrap();
    app.recovery = app_data.recovery;
    Ok(())
}
