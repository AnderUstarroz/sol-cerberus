use anchor_lang::prelude::*;
use errors::*;
use instructions::*;
use state::*;

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

    pub fn delete_rule(
        _ctx: Context<DeleteRule>,
        _role: String,
        _resource: String,
        _permission: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn assign_role(
        ctx: Context<AssignRole>,
        role: String,
        address: Pubkey,
        address_type: AddressType,
    ) -> Result<()> {
        instructions::assign_role::assign_role(ctx, role, address, address_type)
    }

    pub fn delete_assigned_role(
        _ctx: Context<DeleteAssignedRole>,
        _role: String,
        _address: String,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
