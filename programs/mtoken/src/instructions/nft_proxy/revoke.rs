use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use community_managed_token::instruction::create_revoke_instruction;

#[derive(Accounts)]
pub struct RevokeCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    /// CHECK: Checked in cpi
    freeze_authority: UncheckedAccount<'info>,
    #[account(
        constraint = mint_state.mint == mint.key() @ MTokenErrorCode::InvalidMint,
        constraint = mint.key() == from_account.mint @ MTokenErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ MTokenErrorCode::MintStateLocked,
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ MTokenErrorCode::InvalidPolicyMintAssociation,
        constraint = mint.mint_authority == COption::Some(freeze_authority.key()) @ MTokenErrorCode::InvalidPolicyMintAssociation,
        constraint = policy.get_freeze_authority(policy.key()) == freeze_authority.key() @ MTokenErrorCode::InvalidPolicyMintAssociation,
    )]
    mint: Box<Account<'info, Mint>>,
    mint_state: Box<Account<'info, MintState>>,
    from: Signer<'info>,
    #[account(
        mut,
        constraint = from_account.owner == from.key() @ MTokenErrorCode::InvalidTokenAccount,
        constraint = from_account.delegate.is_some() @ MTokenErrorCode::InvalidTokenAccount,
    )]
    from_account: Box<Account<'info, TokenAccount>>,
    token_program: Program<'info, Token>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut RevokeCtx<'_>> for ActionCtx {
    fn from(ctx: &mut RevokeCtx) -> Self {
        ActionCtx {
            action: "revoke".to_string(),
            program_ids: get_program_ids_from_instructions(&ctx.instructions.to_account_info())
                .unwrap(),
            payer: None,
            from: Some(ctx.from.key()),
            from_account: Some(ctx.from_account.clone().into()),
            to: None,
            to_account: None,
            mint: ctx.mint.key(),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner(),
        }
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, RevokeCtx<'info>>) -> Result<()> {
    ctx.accounts.mint_state.assert_unlocked()?;
    let action_ctx: ActionCtx = ctx.accounts.into();
    let policy = &ctx.accounts.policy;
    policy.matches(action_ctx)?;

    invoke_signed(
        &create_revoke_instruction(
            &ctx.accounts.mint.key(),
            &ctx.accounts.from.key(),
            &ctx.accounts.policy.key(),
        )?,
        &[
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.from_account.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.policy.to_account_info(),
            ctx.accounts.freeze_authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.cmt_program.to_account_info(),
        ],
        &[&policy.signer_seeds()],
    )?;

    Ok(())
}
