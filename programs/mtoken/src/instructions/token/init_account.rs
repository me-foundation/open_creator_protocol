use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct InitAccountCtx<'info> {
    /// CHECK: Account created or checked in handler
    mint: UncheckedAccount<'info>,
    /// CHECK: Account created or checked in handler
    #[account(mut)]
    token_account: UncheckedAccount<'info>,
    /// CHECK: Account created or checked in handler
    token_account_owner: UncheckedAccount<'info>,

    #[account(mut)]
    payer: Signer<'info>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitAccountCtx>) -> Result<()> {
    let cpi_accounts = associated_token::Create {
        payer: ctx.accounts.payer.to_account_info(),
        associated_token: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.token_account_owner.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    associated_token::create(cpi_context)?;
    Ok(())
}
