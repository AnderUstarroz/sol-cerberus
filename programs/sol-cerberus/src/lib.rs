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

    pub fn add_rule(ctx: Context<AddRule>, rule_data: RuleData) -> Result<()> {
        instructions::add_rule::add_rule(ctx, rule_data)
    }

    pub fn delete_rule(_ctx: Context<DeleteRule>) -> Result<()> {
        Ok(())
    }

    pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
        instructions::assign_role::assign_role(ctx, assign_role_data)
    }

    pub fn delete_assigned_role(_ctx: Context<DeleteAssignedRole>) -> Result<()> {
        Ok(())
    }

    pub fn allowed(ctx: Context<Allowed>, allowed_params: AllowedRule) -> Result<()> {
        instructions::allowed::allowed(ctx, allowed_params)
    }
}
