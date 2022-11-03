use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRulesetV0Ix {
    pub collector: Pubkey,
    pub name: String,
    pub rule_allow_programs_except_non_pda_owner: bool,
    pub rule_allow_programs_except_cooldown_seconds: u32,
    pub rule_allow_programs: Vec<Pubkey>,
    pub rule_deny_addresses: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: InitRulesetV0Ix)]
pub struct InitRulesetV0Ctx<'info> {
    #[account(
        init,
        payer = payer,
        space = RULESET_SIZE,
        seeds = [RULESET_SEED.as_bytes(), ix.name.as_bytes()],
        constraint = authority.key().to_string() == RULESET_AUTHORITY @ ErrorCode::InvalidAuthority,
        bump,
    )]
    ruleset: Account<'info, Ruleset>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitRulesetV0Ctx>, ix: InitRulesetV0Ix) -> Result<()> {
    let ruleset = &mut ctx.accounts.ruleset;
    ruleset.bump = *ctx.bumps.get("ruleset").unwrap();
    ruleset.version = 0;
    ruleset.authority = ctx.accounts.authority.key();
    ruleset.collector = ix.collector;
    ruleset.name = ix.name;
    ruleset.rule_allow_programs_except_non_pda_owner = ix.rule_allow_programs_except_non_pda_owner;
    ruleset.rule_allow_programs_except_cooldown_seconds =
        ix.rule_allow_programs_except_cooldown_seconds;
    ruleset.rule_allow_programs = ix.rule_allow_programs;
    ruleset.rule_deny_addresses = ix.rule_deny_addresses;
    Ok(())
}
