use crate::state::app::App;
use crate::state::rule::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        has_one = authority,
        seeds = [b"app".as_ref(), app.id.key().as_ref()], 
        bump = app.bump,
    )]
    pub app: Account<'info, App>,
    #[account(
        mut,
        close = collector,
        seeds = [rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), app.id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,

}
