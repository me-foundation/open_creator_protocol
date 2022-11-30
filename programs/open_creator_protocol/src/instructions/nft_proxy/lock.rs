use crate::action::*;
use crate::errors::OCPErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct LockCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    #[account(
        constraint = mint_state.mint == mint.key() @ OCPErrorCode::InvalidMint,
        constraint = mint.key() == from_account.mint @ OCPErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ OCPErrorCode::MintStateLocked,
    )]
    mint: Box<Account<'info, Mint>>,
    /// CHECK: going to check in action ctx
    metadata: UncheckedAccount<'info>,
    #[account(mut)]
    mint_state: Box<Account<'info, MintState>>,
    from: Signer<'info>,
    #[account(
        constraint = from_account.owner == from.key() && from_account.amount == 1 @ OCPErrorCode::InvalidTokenAccount,
        constraint = from_account.delegate == COption::Some(to.key()) @ OCPErrorCode::InvalidTokenAccount,
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
        let mut action_ctx = ActionCtx {
            action: "lock".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            payer: None,
            from: Some(ctx.from.key().to_string()),
            to: Some(ctx.to.key().to_string()),
            mint: ctx.mint.key().to_string(),
            metadata: Some(to_metadata_ctx(&ctx.mint.key(), &ctx.metadata).expect("invalid metadata")),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner().into(),
        };
        action_ctx
            .parse_instructions(&ctx.instructions)
            .expect("failed to parse sysvar instructions");
        action_ctx
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, LockCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    ctx.accounts.policy.matches(&action_ctx)?;

    let mint_state = &mut ctx.accounts.mint_state;
    mint_state.locked_by = Some(ctx.accounts.to.key());

    Ok(())
}
