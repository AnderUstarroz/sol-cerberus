use anchor_lang::prelude::*;
use errors::*;
use instructions::*;
pub use mpl_token_metadata;
pub use sol_cerberus_macros;
use solana_program::pubkey;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

const PROGRAM_AUTHORITY: Pubkey = pubkey!("SCfVPLT34pep4pHfnMTzSyMZ2kLcxjKTGS2phuiApz5");
declare_id!("SCERbrcgSPwgkrJ7j4TABr17dhYzdgiwPZUSSfFPt8x");

#[program]
pub mod sol_cerberus {
    use crate::utils::utc_now;

    use super::*;

    pub fn initialize_app(ctx: Context<InitializeApp>, app_data: AppData) -> Result<()> {
        instructions::initialize_app::initialize_app(ctx, app_data)
    }

    pub fn update_app(ctx: Context<UpdateApp>, app_data: UpdateAppData) -> Result<()> {
        instructions::update_app::update_app(ctx, app_data)
    }

    pub fn delete_app(ctx: Context<DeleteApp>) -> Result<()> {
        emit!(AppChanged {
            time: utc_now(),
            app_id: ctx.accounts.app.id,
            authority: ctx.accounts.app.authority,
        });
        Ok(())
    }

    pub fn add_rule(ctx: Context<AddRule>, rule_data: RuleData) -> Result<()> {
        instructions::add_rule::add_rule(ctx, rule_data)
    }

    pub fn delete_rule(ctx: Context<DeleteRule>) -> Result<()> {
        emit!(RulesChanged {
            time: utc_now(),
            app_id: ctx.accounts.app.id,
        });
        Ok(())
    }

    pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
        instructions::assign_role::assign_role(ctx, assign_role_data)
    }

    pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
        emit!(RolesChanged {
            time: utc_now(),
            app_id: ctx.accounts.app.id,
        });
        Ok(())
    }

    /**
     * Updates the app.updated_at field so clients
     * can keep track and cache permissions.
     */
    pub fn update_cache(ctx: Context<UpdateCache>) -> Result<()> {
        instructions::update_cache::update_cache(ctx)
    }

    pub fn allowed(ctx: Context<Allowed>, allowed_params: AllowedRule) -> Result<()> {
        instructions::allowed::allowed(ctx, allowed_params)
    }
}
