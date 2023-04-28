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

    pub fn delete_app(_ctx: Context<DeleteApp>) -> Result<()> {
        Ok(())
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::update_authority::update_authority(ctx, new_authority)
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

    pub fn allowed(ctx: Context<Allowed>, allowed_params: AllowedRule) -> Result<()> {
        instructions::allowed::allowed(ctx, allowed_params)
    }
}
