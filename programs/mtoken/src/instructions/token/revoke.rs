use anchor_spl::token;
use anchor_spl::token::FreezeAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::Revoke;
use anchor_spl::token::ThawAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RevokeCtx<'info> {
    #[account(mut)]
    mint_manager: Account<'info, MintManager>,
    #[account(constraint = mint.key() == mint_manager.mint @ ErrorCode::InvalidMint)]
    mint: Box<Account<'info, Mint>>,

    #[account(mut, constraint =
        holder_token_account.owner == holder.key()
        && holder_token_account.mint == mint_manager.mint
        && holder_token_account.amount == 1
        && holder_token_account.delegate.is_some()
        @ ErrorCode::InvalidHolderTokenAccount
    )]
    holder_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    holder: Signer<'info>,

    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RevokeCtx>) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    let mint_manager_seeds = &[
        MINT_MANAGER_SEED.as_bytes(),
        mint.as_ref(),
        &[ctx.accounts.mint_manager.bump],
    ];
    let mint_manager_signer = &[&mint_manager_seeds[..]];

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::thaw_account(cpi_context)?;

    let cpi_accounts = Revoke {
        source: ctx.accounts.holder_token_account.to_account_info(),
        authority: ctx.accounts.holder.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::revoke(cpi_context)?;

    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::freeze_account(cpi_context)?;

    Ok(())
}
