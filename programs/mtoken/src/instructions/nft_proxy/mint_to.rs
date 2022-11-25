use crate::errors::MTokenErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::{Mint, Token};
use community_managed_token::instruction::create_mint_to_instruction;

#[derive(Accounts)]
pub struct MintToCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    /// CHECK: Checked in cpi
    freeze_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = mint_state.mint == mint.key() @ MTokenErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ MTokenErrorCode::MintStateLocked,
        constraint = mint.decimals == 0 @ MTokenErrorCode::InvalidMint, // nft
        constraint = mint.supply == 0 @ MTokenErrorCode::InvalidMint, // nft
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ MTokenErrorCode::InvalidPolicyMintAssociation,
        constraint = mint.mint_authority == COption::Some(freeze_authority.key()) @ MTokenErrorCode::InvalidPolicyMintAssociation,
        constraint = policy.get_freeze_authority(policy.key()) == freeze_authority.key() @ MTokenErrorCode::InvalidPolicyMintAssociation,
    )]
    mint: Box<Account<'info, Mint>>,
    /// CHECK: going to check in action ctx
    metadata: UncheckedAccount<'info>,
    mint_state: Box<Account<'info, MintState>>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: Not read from, and checked in cpi
    from: UncheckedAccount<'info>,
    /// CHECK: Not read from, and checked in cpi
    #[account(mut)]
    from_account: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut MintToCtx<'_>> for ActionCtx {
    fn from(ctx: &mut MintToCtx) -> Self {
        ActionCtx {
            action: "mint_to".to_string(),
            program_ids: get_program_ids_from_instructions(&ctx.instructions.to_account_info())
                .unwrap(),
            payer: Some(ctx.payer.key().to_string()),
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
        }
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, MintToCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    let policy = &ctx.accounts.policy;
    policy.matches(action_ctx)?;

    invoke_signed(
        &create_mint_to_instruction(
            &ctx.accounts.mint.key(),
            &ctx.accounts.from.key(),
            &ctx.accounts.policy.key(),
            1,
        )?,
        &[
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.from_account.to_account_info(),
            ctx.accounts.policy.to_account_info(),
            ctx.accounts.freeze_authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.cmt_program.to_account_info(),
        ],
        &[&policy.signer_seeds()],
    )?;

    Ok(())
}
