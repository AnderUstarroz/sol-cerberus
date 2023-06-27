use anchor_lang::prelude::*;
pub use constants::*;
use errors::*;
use instructions::*;
pub use sol_cerberus_macros;
use state::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("SCERbrcgSPwgkrJ7j4TABr17dhYzdgiwPZUSSfFPt8x");

#[program]
pub mod sol_cerberus {

    use super::*;

    pub fn initialize_app(ctx: Context<InitializeApp>, app_data: AppData) -> Result<()> {
        instructions::initialize_app::initialize_app(ctx, app_data)
    }

    pub fn update_app(ctx: Context<UpdateApp>, app_data: UpdateAppData) -> Result<()> {
        instructions::update_app::update_app(ctx, app_data)
    }

    pub fn delete_app(ctx: Context<DeleteApp>) -> Result<()> {
        instructions::delete_app::delete_app(ctx)
    }

    pub fn add_rule(ctx: Context<AddRule>, rule_data: RuleData) -> Result<()> {
        instructions::add_rule::add_rule(ctx, rule_data)
    }

    pub fn delete_rule(ctx: Context<DeleteRule>) -> Result<()> {
        instructions::delete_rule::delete_rule(ctx)
    }

    pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
        instructions::assign_role::assign_role(ctx, assign_role_data)
    }

    pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
        instructions::delete_assigned_role::delete_assigned_role(ctx)
    }

    /**
     * Updates either app.roles_updated_at or app.rules_updated_at fields, so clients
     * can keep track and cache roles & rules accordingly.
     */
    pub fn update_cache(ctx: Context<UpdateCache>, cache_updated: u8) -> Result<()> {
        instructions::update_cache::update_cache(ctx, cache_updated)
    }

    /**
     * Checks if the current user is authorized to run the instruction,
     * throwing "Unauthorized" error otherwise.
     */
    pub fn allowed(ctx: Context<Allowed>, allowed_rule: AllowedRule) -> Result<()> {
        instructions::allowed::allowed(
            &ctx.accounts.signer,
            &ctx.accounts.sol_cerberus_app,
            &ctx.accounts.sol_cerberus_role,
            &ctx.accounts.sol_cerberus_rule,
            &ctx.accounts.sol_cerberus_token,
            &ctx.accounts.sol_cerberus_metadata,
            &mut ctx.accounts.sol_cerberus_seed,
            &ctx.accounts.system_program,
            allowed_rule,
        )
    }
}
