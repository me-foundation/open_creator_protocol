use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct InitPolicyArg {
    pub uuid: Pubkey,
    pub json_rule: String,
}

#[derive(Accounts)]
#[instruction(arg: InitPolicyArg)]
pub struct InitPolicyCtx<'info> {
    #[account(
        init,
        payer = authority,
        space = Policy::LEN,
        seeds = [Policy::SEED.as_bytes(), arg.uuid.as_ref()],
        bump,
    )]
    policy: Box<Account<'info, Policy>>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPolicyCtx>, arg: InitPolicyArg) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.version = 0;
    policy.bump = [*ctx.bumps.get("policy").unwrap()];
    policy.uuid = arg.uuid;
    policy.authority = ctx.accounts.authority.key();
    policy.json_rule = arg.json_rule;
    policy.valid()
}
