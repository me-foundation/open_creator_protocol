use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct UnlockCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    #[account(
        constraint = mint_state.mint == mint.key() @ MTokenErrorCode::InvalidMint,
        constraint = mint_state.locked_by == Some(from.key()) @ MTokenErrorCode::InvalidLockedBy,
    )]
    mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    mint_state: Box<Account<'info, MintState>>,
    from: Signer<'info>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut UnlockCtx<'_>> for ActionCtx {
    fn from(ctx: &mut UnlockCtx) -> Self {
        ActionCtx {
            action: "unlock".to_string(),
            program_ids: get_program_ids_from_instructions(&ctx.instructions.to_account_info())
                .unwrap(),
            payer: None,
            from: Some(ctx.from.key()),
            from_account: None,
            to: None,
            to_account: None,
            mint: ctx.mint.key(),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner(),
        }
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, UnlockCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    let policy = &ctx.accounts.policy;
    policy.matches(action_ctx)?;

    let mint_state = &mut ctx.accounts.mint_state;
    mint_state.locked_by = None;

    Ok(())
}
