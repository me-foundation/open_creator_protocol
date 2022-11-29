use crate::{royalty::DynamicRoyalty, state::*};
use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct InitPolicyArg {
    pub json_rule: Option<String>,
    pub dynamic_royalty: Option<DynamicRoyalty>,
}

#[derive(Accounts)]
#[instruction(arg: InitPolicyArg)]
pub struct InitPolicyCtx<'info> {
    #[account(
        init,
        payer = authority,
        space = Policy::LEN,
        seeds = [Policy::SEED.as_bytes(), uuid.key().as_ref()],
        bump,
    )]
    policy: Box<Account<'info, Policy>>,
    /// CHECK: only used as a random seed
    uuid: UncheckedAccount<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPolicyCtx>, arg: InitPolicyArg) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.version = 0;
    policy.bump = [*ctx.bumps.get("policy").unwrap()];
    policy.uuid = ctx.accounts.uuid.key();
    policy.authority = ctx.accounts.authority.key();
    policy.json_rule = arg.json_rule;
    policy.dynamic_royalty = arg.dynamic_royalty;
    policy.valid()
}
