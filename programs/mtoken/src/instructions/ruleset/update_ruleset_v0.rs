use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateRulesetV0Ix {
    pub authority: Pubkey,
    pub collector: Pubkey,
    pub rule_allow_programs_except_non_pda_owner: bool,
    pub rule_allow_programs_except_cooldown_seconds: u32,
    pub rule_allow_programs: Vec<Pubkey>,
    pub rule_deny_addresses: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct UpdateRulesetV0Ctx<'info> {
    #[account(mut)]
    ruleset: Account<'info, Ruleset>,
    #[account(constraint = authority.key() == ruleset.authority @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateRulesetV0Ctx>, ix: UpdateRulesetV0Ix) -> Result<()> {
    let ruleset = &mut ctx.accounts.ruleset;
    ruleset.authority = ix.authority;
    ruleset.collector = ix.collector;
    ruleset.rule_allow_programs_except_non_pda_owner = ix.rule_allow_programs_except_non_pda_owner;
    ruleset.rule_allow_programs_except_cooldown_seconds =
        ix.rule_allow_programs_except_cooldown_seconds;
    ruleset.rule_allow_programs = ix.rule_allow_programs;
    ruleset.rule_deny_addresses = ix.rule_deny_addresses;
    Ok(())
}
