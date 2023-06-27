use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::instructions::allowed::{allowed, AllowedRule};
use crate::state::app::{App, Seed};
use crate::state::role::Role;
use crate::state::rule::*;
use crate::utils::{utc_now, roles::address_or_wildcard};
use anchor_lang::prelude::*;
use crate::metadata_program;


#[derive(Accounts)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = collector,
        seeds = [rule.namespace.to_le_bytes().as_ref(), rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), sol_cerberus_app.id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
    #[account(
        seeds = [b"app".as_ref(), sol_cerberus_app.id.key().as_ref()],
        bump = sol_cerberus_app.bump,
    )]
    pub sol_cerberus_app: Box<Account<'info, App>>,
    #[account(
        seeds = [sol_cerberus_role.role.as_ref(), address_or_wildcard(&sol_cerberus_role.address), sol_cerberus_role.app_id.key().as_ref()],
        bump = sol_cerberus_role.bump
    )]
    pub sol_cerberus_role: Option<Box<Account<'info, Role>>>,
    #[account(
        seeds = [sol_cerberus_rule.namespace.to_le_bytes().as_ref(), sol_cerberus_rule.role.as_ref(), sol_cerberus_rule.resource.as_ref(), sol_cerberus_rule.permission.as_ref(), sol_cerberus_rule.app_id.key().as_ref()],
        bump = sol_cerberus_rule.bump,
    )]
    pub sol_cerberus_rule: Option<Box<Account<'info, Rule>>>,
    #[account(
        seeds = [sol_cerberus_rule2.namespace.to_le_bytes().as_ref(), sol_cerberus_rule2.role.as_ref(), sol_cerberus_rule2.resource.as_ref(), sol_cerberus_rule2.permission.as_ref(), sol_cerberus_rule2.app_id.key().as_ref()],
        bump = sol_cerberus_rule2.bump,
    )]
    pub sol_cerberus_rule2: Option<Box<Account<'info, Rule>>>,
    #[account()]
    pub sol_cerberus_token: Option<Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", metadata_program::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program =metadata_program::ID,
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

pub fn delete_rule(
    ctx: Context<DeleteRule>
) -> Result<()> {
      // Checks if is allowed to delete a rule for this specific Namespace and Role.
      allowed(
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
            namespace: Namespaces::DeleteRuleNSRole as u8,
            resource: ctx.accounts.rule.namespace.to_string(),
            permission: ctx.accounts.rule.role.to_string(),
        },
    )?;
    // // Checks if is allowed to delete a rule for this specific Resource and Permission.
    allowed(
        &ctx.accounts.signer,
        &ctx.accounts.sol_cerberus_app,
        &ctx.accounts.sol_cerberus_role,
        &ctx.accounts.sol_cerberus_rule2,
        &ctx.accounts.sol_cerberus_token,
        &ctx.accounts.sol_cerberus_metadata,
        &mut None,
        &ctx.accounts.system_program,
        AllowedRule {
            app_id: ctx.accounts.sol_cerberus_app.id.key(),
            namespace: Namespaces::DeleteRuleResourcePerm as u8,
            resource: ctx.accounts.rule.resource.to_string(),
            permission: ctx.accounts.rule.permission.to_string(),
        },
    )?;

    emit!(RulesChanged {
        time: utc_now(),
        app_id: ctx.accounts.sol_cerberus_app.id,
    });
    Ok(())
}
