use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct InitPolicyArg {
    pub update_authority: Pubkey,
    pub update_authority_nonce: u8,
    pub json_rule: String,
}

#[derive(Accounts)]
#[instruction(arg: InitPolicyArg)]
pub struct InitPolicyCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = Policy::LEN,
        seeds = [Policy::SEED.as_bytes(), arg.update_authority.key().as_ref(), &[arg.update_authority_nonce]],
        constraint = authority.key().to_string() == Policy::MANAGED_AUTHORITY @ MTokenErrorCode::InvalidAuthority,
        bump,
    )]
    policy: Account<'info, Policy>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPolicyCtx>, arg: InitPolicyArg) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.version = 0;
    policy.update_authority = arg.update_authority;
    policy.update_authority_nonce = [arg.update_authority_nonce];
    policy.bump = [*ctx.bumps.get("policy").unwrap()];
    policy.json_rule = arg.json_rule;
    policy.valid()
}
