use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use solana_program::pubkey;
use crate::state::App;
use crate::state::rule::Rule;
use crate::utils::{allowed_perm, utc_now, address_or_wildcard, allowed_authority};
use crate::state::role::Role;
use anchor_lang::prelude::*;
use crate::Errors::{Unauthorized, InvalidAppID};

const TEST_APP_ID: Pubkey = pubkey!("testX83crd4vAgRrvmwXgVQ2r69uCpg8xzh8A5X124x");

#[derive(Accounts)]
pub struct Allowed<'info> {
    #[account()]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"app".as_ref(), sol_cerberus_app.id.key().as_ref()], 
        bump = sol_cerberus_app.bump,
    )]
    pub sol_cerberus_app: Box<Account<'info, App>>,
    #[account(
        seeds = [sol_cerberus_rule.namespace.to_le_bytes().as_ref(), sol_cerberus_rule.role.as_ref(), sol_cerberus_rule.resource.as_ref(), sol_cerberus_rule.permission.as_ref(), sol_cerberus_rule.app_id.key().as_ref()], 
        bump = sol_cerberus_rule.bump,
    )]
    pub sol_cerberus_rule: Option< Box<Account<'info, Rule>>>,
    #[account(
        seeds = [sol_cerberus_role.role.as_ref(), address_or_wildcard(&sol_cerberus_role.address), sol_cerberus_role.app_id.key().as_ref()], 
        bump = sol_cerberus_role.bump
    )]
    pub sol_cerberus_role: Option< Box<Account<'info, Role>>>,
    #[account()]
    pub sol_cerberus_token: Option< Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), sol_cerberus_metadata.mint.key().as_ref()],
        seeds::program = mpl_token_metadata::ID,
        bump,
    )]
    pub sol_cerberus_metadata: Option< Box<Account<'info, MetadataAccount>>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AllowedRule {
    pub app_id: Pubkey,
    pub namespace: u8,
    pub resource: String,
    pub permission: String,
}


pub fn allowed(
    signer:&Signer,
    app:&Box<Account<App>>,
    role:&Option<Box<Account<Role>>>,
    rule:&Option<Box<Account<Rule>>>,
    token:&Option<Box<Account<TokenAccount>>>,
    metadata:&Option<Box<Account<MetadataAccount>>>,
    allowed_rule:AllowedRule) -> Result<()> {
    // The APP ID must be the one authorized by the program
    if allowed_rule.app_id != app.id.key(){
        // Ignore APP Check on Test APP
        if allowed_rule.app_id != TEST_APP_ID{
            return Err(error!(InvalidAppID))
        }
    }
    
    // Authority is always allowed
    if allowed_authority(&signer.key(), &app.authority.key()){
        return Ok(());
    }

    // Rule or Role can only be empty when using Authority
    if rule.is_none() || role.is_none(){
        return Err(error!(Unauthorized))
    }

    let rule = rule.as_ref().unwrap();
    let role = role.as_ref().unwrap();

    // The APP ID must match on: APP, Role, Rule
    if app.id != rule.app_id  || app.id != role.app_id{
        return Err(error!(Unauthorized))
    }

    // Check Rule is within the corresponding Namespace
    if rule.namespace != allowed_rule.namespace  {
        return Err(error!(Unauthorized))
    }

    // Check Resource & Permission
    if !allowed_perm(&allowed_rule.resource, &rule.resource) || !allowed_perm(&allowed_rule.permission, &rule.permission){
        return Err(error!(Unauthorized))
    }

    // Check Role
    if role.role != rule.role {
        return Err(error!(Unauthorized))
    }

    let now = utc_now();
    // Check if role expired
    if role.expires_at.is_some() && role.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if rule expired 
    if rule.expires_at.is_some() && rule.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if the wallet is authorized (Address = "None" is considered wildcard "*")
      if role.address.is_none() || signer.key() == role.address.unwrap(){
        return Ok(());
    }
    // Check if the NFT or Collection Mint addresses are authorized
    if token.is_some(){
        let token = token.as_ref().unwrap();
        // Check if is the real owner of the NFT and has at least one
        if token.owner != signer.key() || token.amount <= 0{
            return Err(error!(Unauthorized))
        }
        // NFT authorized (Address = "None" is considered wildcard "*")
        if role.address.is_none() || token.mint == role.address.unwrap(){
            return Ok(());
        }
        if  metadata.is_some() {
            let metadata = metadata.as_ref().unwrap();
            if metadata.collection.is_some() && metadata.mint == token.mint {
                let collection = metadata.collection.as_ref().unwrap();
                // Collection authorized (Address = "None" is considered wildcard "*")
                if collection.verified && (role.address.is_none() || collection.key == role.address.unwrap()){
                    return Ok(());
                }
            }
        }
    }

    Err(error!(Unauthorized))
}

