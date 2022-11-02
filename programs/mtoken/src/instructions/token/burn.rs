use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Burn;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::ThawAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self};

#[derive(Accounts)]
pub struct BurnCtx<'info> {
    #[account(mut, close = holder, constraint = mint_manager.mint == mint.key() @ ErrorCode::InvalidMint)]
    mint_manager: Account<'info, MintManager>,
    #[account(mut)]
    mint: Account<'info, Mint>,

    #[account(mut, constraint =
        holder_token_account.owner == holder.key()
        && holder_token_account.mint == mint.key()
        && holder_token_account.amount == 1 @ ErrorCode::InvlaidHolderTokenAccount)]
    holder_token_account: Account<'info, TokenAccount>,
    holder: Signer<'info>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BurnCtx>) -> Result<()> {
    let mint_manager_key = ctx.accounts.mint.key();
    let mint_manager_seeds = &[
        MINT_MANAGER_SEED.as_bytes(),
        mint_manager_key.as_ref(),
        &[ctx.accounts.mint_manager.bump],
    ];
    let mint_manager_signer = &[&mint_manager_seeds[..]];

    if ctx.accounts.mint.supply > 1
        || ctx.accounts.mint.supply != ctx.accounts.holder_token_account.amount
    {
        return Err(error!(ErrorCode::InvalidMint));
    }

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::thaw_account(cpi_context)?;

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.holder_token_account.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::burn(cpi_context, 1)?;

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.holder_token_account.to_account_info(),
        destination: ctx.accounts.holder.to_account_info(),
        authority: ctx.accounts.holder.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::close_account(cpi_context)?;

    Ok(())
}
