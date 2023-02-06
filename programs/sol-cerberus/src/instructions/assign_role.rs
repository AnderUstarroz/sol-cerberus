use crate::state::app::App;
use crate::state::assigned_role::*;
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
#[instruction(role: String, address:Pubkey)]
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
        seeds = [role.as_ref(), address.key().as_ref()], 
        constraint = valid_rule(false, &role)  @ Errors::InvalidRole,
        bump
    )]
    pub assigned_role: Account<'info, AssignedRole>,
    pub system_program: Program<'info, System>,
}

pub fn assign_role(
    ctx: Context<AssignRole>,
    role: String,
    address: Pubkey,
    address_type: AddressType,
) -> Result<()> {
    let assigned_role = &mut ctx.accounts.assigned_role;
    assigned_role.bump = *ctx.bumps.get("assigned_role").unwrap();
    assigned_role.app_id = ctx.accounts.app.id;
    assigned_role.address = address;
    assigned_role.role = role;
    assigned_role.address_type = address_type;
    Ok(())
}
