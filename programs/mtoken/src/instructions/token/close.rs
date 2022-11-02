// giannis
use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::ThawAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self};

#[derive(Accounts)]
pub struct CloseCtx<'info> {
    mint_manager: Account<'info, MintManager>,
    /// CHECK: Account is not read from
    #[account(mut)]
    mint: UncheckedAccount<'info>,

    #[account(mut, constraint = token_account.owner == owner.key() @ ErrorCode::InvalidCloseTokenAccount)]
    token_account: Account<'info, TokenAccount>,
    owner: Signer<'info>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseCtx>) -> Result<()> {
    if ctx.accounts.token_account.is_frozen() {
        let mint_manager_key = ctx.accounts.mint.key();
        let mint_manager_seeds = &[
            MINT_MANAGER_SEED.as_bytes(),
            mint_manager_key.as_ref(),
            &[ctx.accounts.mint_manager.bump],
        ];
        let mint_manager_signer = &[&mint_manager_seeds[..]];

        let cpi_accounts = ThawAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.mint_manager.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context =
            CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
        token::thaw_account(cpi_context)?;
    }

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.token_account.to_account_info(),
        destination: ctx.accounts.owner.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::close_account(cpi_context)?;

    Ok(())
}
