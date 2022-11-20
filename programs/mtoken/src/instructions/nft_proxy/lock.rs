use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct LockCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    #[account(
        constraint = mint_state.mint == mint.key() @ MTokenErrorCode::InvalidMint,
        constraint = mint.key() == from_account.mint @ MTokenErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ MTokenErrorCode::MintStateLocked,
    )]
    mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    mint_state: Box<Account<'info, MintState>>,
    from: Signer<'info>,
    #[account(constraint =
        from_account.owner == from.key()
        && from_account.amount == 1
        && from_account.delegate.is_none()
        @ MTokenErrorCode::InvalidTokenAccount
    )]
    from_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Account is not read from
    to: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut LockCtx<'_>> for ActionCtx {
    fn from(ctx: &mut LockCtx) -> Self {
        ActionCtx {
            action: "lock".to_string(),
            program_ids: get_program_ids_from_instructions(&ctx.instructions.to_account_info())
                .unwrap(),
            from: Some(ctx.from.key()),
            from_account: Some(ctx.from_account.clone().into()),
            to: Some(ctx.to.key()),
            to_account: None,
            mint: ctx.mint.key(),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner(),
        }
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, LockCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    let policy = &ctx.accounts.policy;
    policy.matches(action_ctx)?;

    let mint_state = &mut ctx.accounts.mint_state;
    mint_state.locked_by = Some(ctx.accounts.to.key());

    Ok(())
}
