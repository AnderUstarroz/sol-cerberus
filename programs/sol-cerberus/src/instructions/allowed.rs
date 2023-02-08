use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use mpl_token_metadata::{
    ID as MPL_TOKEN_METADATA_ID,
};
use crate::state::rule::Rule;
use crate::utils::allowed_rule;
use crate::state::role::Role;
use anchor_lang::prelude::*;
use crate::Errors::Unauthorized;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AllowedData {
    pub resource: String,
    pub permission: String,
}

#[derive(Accounts)]
pub struct Allowed<'info> {
    #[account()]
    pub signer: Signer<'info>,
    #[account(
        constraint = token_account.mint == metadata.as_ref().unwrap().mint @ Unauthorized, // Ensure Metadata and NFT accounts belongs to the same token.
        constraint = token_account.owner == signer.key() @ Unauthorized // Ensure NFT owner is the signer.
    )]
    pub token_account: Option<Account<'info, TokenAccount>>,
    #[account(
        seeds = [b"metadata", MPL_TOKEN_METADATA_ID.as_ref(), metadata.mint.key().as_ref()],
        seeds::program = mpl_token_metadata::ID,
        bump,
    )]
    pub metadata: Option<Account<'info, MetadataAccount>>,
    #[account(
        seeds = [rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), rule.app_id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
    #[account(
        seeds = [role.role.as_ref(), role.address.key().as_ref()], 
        bump = role.bump,
        constraint = role.role == rule.role @ Unauthorized, // Ensure Role assigned and Rule's Role are same.
    )]
    pub role: Account<'info, Role>,
    pub system_program: Program<'info, System>,
}

pub fn allowed(ctx: Context<Allowed>, allowed_data:AllowedData) -> Result<()> {
    let token_account = &ctx.accounts.token_account;
    let metadata = &ctx.accounts.metadata;
    let rule = &ctx.accounts.rule;
    let role = &ctx.accounts.role;
    // Run permission check
    if !allowed_rule(&allowed_data.resource, &rule.resource) || !allowed_rule(&allowed_data.permission, &rule.permission){
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

