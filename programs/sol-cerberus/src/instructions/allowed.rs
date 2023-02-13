use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use mpl_token_metadata::{
    ID as MPL_TOKEN_METADATA_ID,
};
use crate::state::App;
use crate::state::rule::Rule;
use crate::utils::{allowed_perm, utc_now};
use crate::state::role::Role;
use anchor_lang::prelude::*;
use crate::Errors::Unauthorized;


#[derive(Accounts)]
pub struct Allowed<'info> {
    #[account()]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"app".as_ref(), sol_cerberus_app.id.key().as_ref()], 
        bump = sol_cerberus_app.bump,
    )]
    pub sol_cerberus_app: Account<'info, App>,
    #[account(
        seeds = [sol_cerberus_rule.role.as_ref(), sol_cerberus_rule.resource.as_ref(), sol_cerberus_rule.permission.as_ref(), sol_cerberus_rule.app_id.key().as_ref()], 
        bump = sol_cerberus_rule.bump,
    )]
    pub sol_cerberus_rule: Account<'info, Rule>,
    #[account(
        seeds = [sol_cerberus_role.role.as_ref(), sol_cerberus_role.address.key().as_ref()], 
        bump = sol_cerberus_role.bump,
        constraint = sol_cerberus_role.role == sol_cerberus_rule.role @ Unauthorized, // Ensure Role assigned and Rule's Role are same.
    )]
    pub sol_cerberus_role: Option<Account<'info, Role>>,
    #[account(
        constraint = sol_cerberus_token_acc.mint == sol_cerberus_metadata.as_ref().unwrap().mint @ Unauthorized, // Ensure Metadata and NFT accounts belongs to the same token.
        constraint = sol_cerberus_token_acc.owner == signer.key() @ Unauthorized // Ensure NFT owner is the signer.
    )]
    pub sol_cerberus_token_acc: Option<Account<'info, TokenAccount>>,
    #[account(
        seeds = [b"metadata", MPL_TOKEN_METADATA_ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program = mpl_token_metadata::ID,
        bump,
    )]
    pub sol_cerberus_metadata: Option<Account<'info, MetadataAccount>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AllowedRule {
    pub resource: String,
    pub permission: String,
}


pub fn allowed(ctx: Context<Allowed>, allowed_data:AllowedRule) -> Result<()> {
    let token_account = &ctx.accounts.sol_cerberus_token_acc;
    let metadata = &ctx.accounts.sol_cerberus_metadata;
    let rule = &ctx.accounts.sol_cerberus_rule;
    // Authority is always allowed
    if &ctx.accounts.signer.key() == &ctx.accounts.sol_cerberus_app.authority.key(){
        return Ok(());
    }
    // Role can only be empty when using Authority
    if ctx.accounts.sol_cerberus_role.is_none(){
        return Err(error!(Unauthorized))
    }

    let role = &ctx.accounts.sol_cerberus_role.as_ref().unwrap();

    // Run permission check
    if !allowed_perm(&allowed_data.resource, &rule.resource) || !allowed_perm(&allowed_data.permission, &rule.permission){
        return Err(error!(Unauthorized))
    }
    // Check if Role has expired.
    let now = utc_now();
    if role.expires_at.is_some() && role.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if Permission has expired.
    if rule.expires_at.is_some() && rule.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if the wallet is authorized
    if ctx.accounts.signer.key() == role.address{
        return Ok(());
    }
    // Check if the NFT Mint address is authorized
    if token_account.is_some() && token_account.as_ref().unwrap().mint == role.address{
        return Ok(());
    }
    // Check if the NFT Collection address is authorized
    if  metadata.is_some() && metadata.as_ref().unwrap().collection.is_some(){
        let collection = metadata.as_ref().unwrap().collection.as_ref().unwrap();
        if collection.verified && collection.key == role.address{
            return Ok(());
        }
     }

    Err(error!(Unauthorized))
}

