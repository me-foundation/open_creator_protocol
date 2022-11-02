use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRulesetIx {
    pub check_seller_fee_basis_points: bool,
    pub name: String,
    pub collector: Pubkey,
    pub disallowed_addresses: Vec<Pubkey>,
    pub allowed_programs: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(ix: InitRulesetIx)]
pub struct InitRulesetCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = RULESET_SIZE,
        seeds = [RULESET_SEED.as_bytes(), ix.name.as_bytes()],
        bump,
    )]
    ruleset: Account<'info, Ruleset>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitRulesetCtx>, ix: InitRulesetIx) -> Result<()> {
    let ruleset = &mut ctx.accounts.ruleset;
    ruleset.bump = *ctx.bumps.get("ruleset").unwrap();
    ruleset.version = 0;
    ruleset.authority = ctx.accounts.authority.key();
    ruleset.collector = ix.collector;
    ruleset.check_seller_fee_basis_points = ix.check_seller_fee_basis_points;
    ruleset.name = ix.name;
    ruleset.allowed_programs = ix.allowed_programs;
    ruleset.disallowed_addresses = ix.disallowed_addresses;
    Ok(())
}
