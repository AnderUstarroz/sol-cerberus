use crate::state::app::App;
use crate::state::role::*;
use crate::utils::rules::*;
use crate::Errors;
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 app_id (Pubkey)
// + 32 address (Pubkey)
// + 20 role (string)
// + 1 + 1 address_type
// + 1 + 8 expires_at Option<i64>
// + 1 bump
// total = 8 + 32 + 32 + 20 + 1 + 1 + 1 + 8 + 1 = 104
#[derive(Accounts)]
#[instruction(assign_role_data:AssignRoleData)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        init,
        payer = authority,
        space = 104,
        seeds = [assign_role_data.role.as_ref(), assign_role_data.address.key().as_ref()], 
        constraint = valid_rule(&assign_role_data.role, false)  @ Errors::InvalidRole,
        bump
    )]
    pub role: Account<'info, Role>,
    pub system_program: Program<'info, System>,
}

pub fn assign_role(
    ctx: Context<AssignRole>,
    assign_role_data: AssignRoleData) -> Result<()> {
    let role = &mut ctx.accounts.role;
    role.bump = *ctx.bumps.get("role").unwrap();
    role.app_id = ctx.accounts.app.id;
    role.address = assign_role_data.address;
    role.role = assign_role_data.role;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;
    Ok(())
}
