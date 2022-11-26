use crate::action_ctx::*;
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
    /// CHECK: going to check in action ctx
    metadata: UncheckedAccount<'info>,
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
        let mut action_ctx = ActionCtx {
            action: "unlock".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            payer: None,
            from: Some(ctx.from.key().to_string()),
            from_is_on_curve: Some(ctx.from.key().is_on_curve()),
            from_account: None,
            to: None,
            to_is_on_curve: None,
            to_account: None,
            mint: ctx.mint.key().to_string(),
            metadata: Some(
                to_metadata_ctx(&ctx.mint.key(), &ctx.metadata).expect("invalid metadata"),
            ),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner().into(),
        };
        action_ctx
            .parse_instructions(&ctx.instructions)
            .expect("failed to parse sysvar instructions");
        action_ctx
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, UnlockCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    ctx.accounts.policy.matches(&action_ctx)?;

    let mint_state = &mut ctx.accounts.mint_state;
    mint_state.locked_by = None;

    Ok(())
}
