use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::instructions::allowed::{allowed, AllowedRule};
use crate::state::app::App;
use crate::state::role::Role;
use crate::state::rule::*;
use crate::utils::{utc_now, roles::address_or_wildcard};
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = collector,
        seeds = [rule.namespace.to_le_bytes().as_ref(), rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), sol_cerberus_app.id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
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
        seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program =mpl_token_metadata::ID,
        bump,
    )]
    pub sol_cerberus_metadata: Option<Box<Account<'info, MetadataAccount>>>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,

}

pub fn delete_rule(
    ctx: Context<DeleteRule>
) -> Result<()> {
    for (n, value) in [
        [&ctx.accounts.rule.namespace.to_string(), &ctx.accounts.rule.role],  // Checks for Namespace and Role
        [&ctx.accounts.rule.resource, &ctx.accounts.rule.permission],   // Checks for Resource and Permission
        ].iter().enumerate() {
        let _ = allowed(
            &ctx.accounts.authority,
            &ctx.accounts.sol_cerberus_app,
            &ctx.accounts.sol_cerberus_role,
            if 0 == n { &ctx.accounts.sol_cerberus_rule } else { &ctx.accounts.sol_cerberus_rule2 },
            &ctx.accounts.sol_cerberus_token,
            &ctx.accounts.sol_cerberus_metadata,
            AllowedRule {
                app_id: ctx.accounts.sol_cerberus_app.id.key(),
                namespace: if 0 == n { Namespaces::DeleteRuleNSRole as u8 } else { Namespaces::DeleteRuleResourcePerm as u8 },
                resource: value[0].to_string(),
                permission: value[1].to_string(),
            },
        );
    }
    emit!(RulesChanged {
        time: utc_now(),
        app_id: ctx.accounts.sol_cerberus_app.id,
    });
    Ok(())
}
