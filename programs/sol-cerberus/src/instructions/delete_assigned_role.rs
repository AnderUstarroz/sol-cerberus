use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::instructions::allowed::{allowed, AllowedRule};
use crate::state::app::{App, Seed};
use crate::state::rule::Namespaces;
use crate::state::role::{Role, RolesChanged};
use crate::utils::{utc_now, roles::address_or_wildcard};
use anchor_lang::prelude::*;
use crate::state::rule::Rule;

#[derive(Accounts)]
pub struct DeleteAssignedRole<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = collector,
        seeds = [role.role.as_ref(), role.address.unwrap().as_ref(), sol_cerberus_app.id.key().as_ref()], 
        bump = role.bump,
    )]
    pub role: Account<'info, Role>,
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
    #[account(
        init_if_needed,
        payer = signer,
        space = 9, // Account discriminator + initialized
        seeds = [b"seed".as_ref(), signer.key.as_ref()],
        bump
    )]
    pub sol_cerberus_seed: Option<Account<'info, Seed>>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


pub fn delete_assigned_role(
    ctx: Context<DeleteAssignedRole>
) -> Result<()> {
    let _ = allowed(
        &ctx.accounts.signer,
        &ctx.accounts.sol_cerberus_app,
        &ctx.accounts.sol_cerberus_role,
        &ctx.accounts.sol_cerberus_rule,
        &ctx.accounts.sol_cerberus_token,
        &ctx.accounts.sol_cerberus_metadata,
        &mut ctx.accounts.sol_cerberus_seed,
        &ctx.accounts.system_program,
        AllowedRule {
            app_id: ctx.accounts.sol_cerberus_app.id.key(),
            namespace: Namespaces::DeleteAssignRole as u8,
            resource: ctx.accounts.role.address_type.to_string(),
            permission: ctx.accounts.role.role.clone(),
        },
    );

    emit!(RolesChanged {
        time: utc_now(),
        app_id: ctx.accounts.sol_cerberus_app.id,
    });
    Ok(())
}
