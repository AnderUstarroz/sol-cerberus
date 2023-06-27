use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::instructions::allowed::{allowed, AllowedRule};
use crate::utils::{valid_rules, utc_now, validate_ns_permission, roles::address_or_wildcard};
use crate::state::role::Role;
use anchor_lang::prelude::*;
use crate::state::app::{App, Seed};
use crate::state::rule::*;
use crate::Errors;
use crate::metadata_program;

// SPACE SIZE:
// + 8 discriminator
// + 32 app_id (Pubkey)
// + 1 namespace (u8)
// + 4 + 16 role (string)
// + 4 + 16 resource (string)
// + 4 + 16  permission (string)
// + 1 + 8 expires_at Option<i64>
// + 1 bump
// total = 8 + 32 + 1 + 4 + 16 + 4 + 16 + 4 + 16  + 1 + 8 + 1 = 111
#[derive(Accounts)]
#[instruction(rule_data:RuleData)]
pub struct AddRule<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 111,
        seeds = [rule_data.namespace.to_le_bytes().as_ref(), rule_data.role.as_ref(), rule_data.resource.as_ref(), rule_data.permission.as_ref(), sol_cerberus_app.id.key().as_ref()], 
        constraint = valid_rules(&rule_data.role, &rule_data.resource, &rule_data.permission)  @ Errors::InvalidRule,
        bump
    )]
    pub rule: Account<'info, Rule>,
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
    #[account(
        seeds = [sol_cerberus_rule2.namespace.to_le_bytes().as_ref(), sol_cerberus_rule2.role.as_ref(), sol_cerberus_rule2.resource.as_ref(), sol_cerberus_rule2.permission.as_ref(), sol_cerberus_rule2.app_id.key().as_ref()],
        bump = sol_cerberus_rule2.bump,
    )]
    pub sol_cerberus_rule2: Option<Box<Account<'info, Rule>>>,
    #[account()]
    pub sol_cerberus_token: Option<Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", metadata_program::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program = metadata_program::ID,
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
    pub system_program: Program<'info, System>,
}

pub fn add_rule(
    ctx: Context<AddRule>,
    data:RuleData
) -> Result<()> {
    // Checks if is allowed to add a rule for this specific Namespace and Role.
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
            namespace: Namespaces::AddRuleNSRole as u8,
            resource: data.namespace.to_string(),
            permission: data.role.to_string(),
        },
    )?;
    // // Checks if is allowed to add a rule for this specific Resource and Permission.
    allowed(
        &ctx.accounts.signer,
        &ctx.accounts.sol_cerberus_app,
        &ctx.accounts.sol_cerberus_role,
        &ctx.accounts.sol_cerberus_rule2,
        &ctx.accounts.sol_cerberus_token,
        &ctx.accounts.sol_cerberus_metadata,
        &mut ctx.accounts.sol_cerberus_seed,
        &ctx.accounts.system_program,
        AllowedRule {
            app_id: ctx.accounts.sol_cerberus_app.id.key(),
            namespace: Namespaces::AddRuleResourcePerm as u8,
            resource: data.resource.to_string(),
            permission: data.permission.to_string(),
        },
    )?;

    // Validate AddressType when creating "AssignRole" or "DeleteAssignRole" rules (Resource can only be Wallet, Nft, Collection or wildcard "*")
    if data.namespace >= Namespaces::AssignRole as u8 && data.namespace <= Namespaces::DeleteAssignRole as u8 {
        if !matches!(data.resource.as_str(), "Wallet" | "Nft" | "Collection" | "*") {
                return Err(error!(Errors::InvalidAddressType))
        }
    }
    
    // Validate Namespace when creating "AddRuleNSRole", "DeleteRuleNSRole" rules.
    // The allowed namespace must be either an u8 number (0-255) or a wildcard "*"
    if data.namespace == Namespaces::AddRuleNSRole as u8 && data.namespace == Namespaces::DeleteRuleNSRole as u8 {
        validate_ns_permission(&data.resource)?;
    }

    // Add permission
    let rule = &mut ctx.accounts.rule;
    rule.bump = *ctx.bumps.get("rule").unwrap();
    rule.app_id = ctx.accounts.sol_cerberus_app.id;
    rule.namespace = data.namespace;
    rule.role = data.role;
    rule.resource = data.resource;
    rule.permission = data.permission;
    rule.expires_at = data.expires_at;
    emit!(RulesChanged {
        time: utc_now(),
        app_id: ctx.accounts.sol_cerberus_app.id,
    });
    Ok(())
}
