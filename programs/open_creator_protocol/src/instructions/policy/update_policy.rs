use crate::errors::OCPErrorCode;
use crate::royalty::DynamicRoyalty;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePolicyArg {
    pub authority: Pubkey,
    pub json_rule: Option<String>,               // None will overwrite the existing field
    pub dynamic_royalty: Option<DynamicRoyalty>, // None will overwrite the existing field
}

#[derive(Accounts)]
#[instruction(arg: UpdatePolicyArg)]
pub struct UpdatePolicyCtx<'info> {
    #[account(mut)]
    policy: Box<Account<'info, Policy>>,
    #[account(
        constraint = (
            authority.key() == policy.authority ||
            authority.key().to_string() == Policy::MANAGED_AUTHORITY
        ) @ OCPErrorCode::InvalidAuthority,
        constraint = (
            arg.authority.to_string() != Policy::MANAGED_AUTHORITY ||
            authority.key().to_string() == Policy::MANAGED_AUTHORITY
        ) @ OCPErrorCode::InvalidAuthority,
    )]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdatePolicyCtx>, arg: UpdatePolicyArg) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.json_rule = arg.json_rule;
    policy.dynamic_royalty = arg.dynamic_royalty;
    policy.authority = arg.authority;
    policy.valid()
}
