use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePolicyArg {
    pub json_rule: String,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct UpdatePolicyCtx<'info> {
    #[account(mut)]
    policy: Account<'info, Policy>,
    #[account(
        constraint = (
            authority.key() == policy.authority ||
            authority.key().to_string() == Policy::MANAGED_AUTHORITY
        ) @ MTokenErrorCode::InvalidAuthority,
    )]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdatePolicyCtx>, arg: UpdatePolicyArg) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.json_rule = arg.json_rule;
    policy.authority = arg.authority;
    policy.valid()
}
