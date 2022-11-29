use crate::{errors::OCPErrorCode, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePolicyCtx<'info> {
    #[account(mut, close=authority)]
    policy: Box<Account<'info, Policy>>,
    #[account(constraint = authority.key() == policy.authority @ OCPErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClosePolicyCtx>) -> Result<()> {
    // zero out the policy
    let policy = &mut ctx.accounts.policy;
    policy.json_rule = None;
    policy.dynamic_royalty = None;
    policy.authority = Pubkey::default();

    Ok(())
}
