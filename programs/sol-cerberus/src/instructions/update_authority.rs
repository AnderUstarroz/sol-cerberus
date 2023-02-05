use crate::state::app::*;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
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

pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    let app = &mut ctx.accounts.app;
    app.authority = new_authority;
    Ok(())
}
