use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::SetAuthority;
use anchor_spl::token::Token;
use anchor_spl::token::{self};
use solana_program::program::invoke;
use solana_program::system_instruction::transfer;
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct InitMintManagerCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = MINT_MANAGER_SIZE,
        seeds = [MINT_MANAGER_SEED.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    mint_manager: Account<'info, MintManager>,
    #[account(mut)]
    mint: Account<'info, Mint>,
    ruleset: Account<'info, Ruleset>,

    /// CHECK: Account is not read from
    #[account(mut, constraint = collector.key() == ruleset.collector @ ErrorCode::InvalidCollector)]
    collector: UncheckedAccount<'info>,
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitMintManagerCtx>) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.bump = *ctx.bumps.get("mint_manager").unwrap();
    mint_manager.version = 0;
    mint_manager.authority = ctx.accounts.authority.key();
    mint_manager.mint = ctx.accounts.mint.key();
    mint_manager.ruleset = ctx.accounts.ruleset.key();

    // check if the mint is nft
    if ctx.accounts.mint.supply != 1 || ctx.accounts.mint.decimals != 0 {
        return Err(error!(ErrorCode::InvalidMint));
    }
    // set mint authoriy
    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::set_authority(
        cpi_context,
        AuthorityType::MintTokens,
        Some(ctx.accounts.mint_manager.key()),
    )?;

    // set freeze authoriy
    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::set_authority(
        cpi_context,
        AuthorityType::FreezeAccount,
        Some(ctx.accounts.mint_manager.key()),
    )?;

    // creation
    invoke(
        &transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.collector.key(),
            CREATION_LAMPORTS,
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.collector.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    Ok(())
}
