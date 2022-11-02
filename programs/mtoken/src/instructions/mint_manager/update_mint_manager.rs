use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::system_instruction::transfer;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateMintManagerIx {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateMintManagerCtx<'info> {
    #[account(mut)]
    mint_manager: Account<'info, MintManager>,
    ruleset: Account<'info, Ruleset>,
    /// CHECK: Account is not read from
    #[account(mut, constraint = collector.key() == ruleset.collector @ ErrorCode::InvalidCollector)]
    collector: UncheckedAccount<'info>,
    #[account(constraint = authority.key() == mint_manager.authority @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateMintManagerCtx>, ix: UpdateMintManagerIx) -> Result<()> {
    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.authority = ix.authority;
    mint_manager.ruleset = ctx.accounts.ruleset.key();

    // update
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
