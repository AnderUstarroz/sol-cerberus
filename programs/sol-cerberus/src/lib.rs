use anchor_lang::prelude::*;
use errors::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

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
}

#[derive(Accounts)]
pub struct Initialize {}
