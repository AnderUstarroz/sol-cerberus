use anchor_lang::prelude::*;
use errors::*;
use instructions::*;
use state::*;
use utils::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_cerberus {
    use super::*;

    pub fn initialize_app(ctx: Context<InitializeApp>, app_data: AppData) -> Result<()> {
        instructions::initialize_app::initialize_app(ctx, app_data)
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::update_authority::update_authority(ctx, new_authority)
    }

    pub fn add_rule(
        ctx: Context<AddRule>,
        role: String,
        resource: String,
        permission: String,
    ) -> Result<()> {
        instructions::add_rule::add_rule(ctx, role, resource, permission)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
