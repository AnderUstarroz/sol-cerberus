use crate::instructions::allowed::{allowed, AllowedRule};
use crate::state::app::App;
use crate::state::role::*;
use crate::state::rule::{Namespaces, Rule};
use crate::utils::{roles::address_or_wildcard, rules::*, utc_now};
use crate::Errors::InvalidRole;
use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};

// SPACE SIZE:
// + 8 discriminator
// + 32 app_id (Pubkey)
// + 32 address (Pubkey)
// + 20 role (string)
// + 1 + 1 address_type
// + 1 + 8 expires_at Option<i64>
// + 1 bump
// total = 8 + 32 + 32 + 20 + 1 + 1 +  1 + 8 + 1 = 104
#[derive(Accounts)]
#[instruction(assign_role_data:AssignRoleData)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 104,
        seeds = [assign_role_data.role.as_ref(), address_or_wildcard(&assign_role_data.address), sol_cerberus_app.id.key().as_ref()],
        constraint = valid_rule(&assign_role_data.role, true)  @ InvalidRole,
        bump
    )]
    pub role: Account<'info, Role>,
    #[account(
        seeds = [b"app".as_ref(), sol_cerberus_app.id.key().as_ref()],
        bump = sol_cerberus_app.bump,
    )]
    pub sol_cerberus_app: Box<Account<'info, App>>,
    #[account(
        seeds = [sol_cerberus_role.role.as_ref(),  address_or_wildcard(&sol_cerberus_role.address), sol_cerberus_role.app_id.key().as_ref()],
        bump = sol_cerberus_role.bump
    )]
    pub sol_cerberus_role: Option<Box<Account<'info, Role>>>,
    #[account(
        seeds = [sol_cerberus_rule.namespace.to_le_bytes().as_ref(), sol_cerberus_rule.role.as_ref(), sol_cerberus_rule.resource.as_ref(), sol_cerberus_rule.permission.as_ref(), sol_cerberus_rule.app_id.key().as_ref()],
        bump = sol_cerberus_rule.bump,
    )]
    pub sol_cerberus_rule: Option<Box<Account<'info, Rule>>>,
    #[account()]
    pub sol_cerberus_token: Option<Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program =mpl_token_metadata::ID,
        bump,
    )]
    pub sol_cerberus_metadata: Option<Box<Account<'info, MetadataAccount>>>,
    pub system_program: Program<'info, System>,
}

pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
    let _ = allowed(
        &ctx.accounts.authority,
        &ctx.accounts.sol_cerberus_app,
        &ctx.accounts.sol_cerberus_role,
        &ctx.accounts.sol_cerberus_rule,
        &ctx.accounts.sol_cerberus_token,
        &ctx.accounts.sol_cerberus_metadata,
        AllowedRule {
            app_id: ctx.accounts.sol_cerberus_app.id.key(),
            namespace: Namespaces::AssignRole as u8,
            resource: assign_role_data.address_type.to_string(),
            permission: assign_role_data.role.clone(),
        },
    )?;

    let role = &mut ctx.accounts.role;
    role.bump = *ctx.bumps.get("role").unwrap();
    role.app_id = ctx.accounts.sol_cerberus_app.id;
    role.address = assign_role_data.address;
    role.role = assign_role_data.role;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;
    emit!(RolesChanged {
        time: utc_now(),
        app_id: ctx.accounts.sol_cerberus_app.id,
    });
    Ok(())
}
