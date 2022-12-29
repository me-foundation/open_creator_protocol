use crate::action::*;
use crate::errors::OCPErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::solana_program::sysvar;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::MetadataAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use community_managed_token::instruction::create_initialize_account_instruction;

#[derive(Accounts)]
pub struct InitAccountCtx<'info> {
    policy: Box<Account<'info, Policy>>,
    /// CHECK: Checked in cpi
    freeze_authority: UncheckedAccount<'info>,
    #[account(
        constraint = mint_state.mint == mint.key() @ OCPErrorCode::InvalidMint,
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = mint.mint_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = mint_state.policy == policy.key() @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = policy.get_freeze_authority(policy.key()) == freeze_authority.key() @ OCPErrorCode::InvalidPolicyMintAssociation,
    )]
    mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"metadata", anchor_spl::metadata::Metadata::id().as_ref(), mint.key().as_ref()],
        seeds::program = anchor_spl::metadata::Metadata::id(),
        bump,
    )]
    metadata: Box<Account<'info, MetadataAccount>>,
    #[account(mut)]
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
    associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut InitAccountCtx<'_>> for ActionCtx {
    fn from(ctx: &mut InitAccountCtx) -> Self {
        let mut action_ctx = ActionCtx {
            action: "init_account".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            payer: Some(ctx.payer.key().to_string()),
            from: Some(ctx.from.key().to_string()),
            to: None,
            mint: ctx.mint.key().to_string(),
            metadata: Some(ctx.metadata.clone().into()),
            mint_account: Some(ctx.mint.clone().into()),
            mint_state: ctx.mint_state.clone().into_inner().into(),
        };
        action_ctx
            .parse_instructions(&ctx.instructions)
            .expect("failed to parse sysvar instructions");
        action_ctx
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, InitAccountCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    ctx.accounts.policy.matches(&action_ctx)?;

    invoke_signed(
        &create_initialize_account_instruction(
            &ctx.accounts.mint.key(),
            &ctx.accounts.from.key(),
            &ctx.accounts.payer.key(),
            &ctx.accounts.policy.key(),
        )?,
        &[
            ctx.accounts.from_account.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.policy.to_account_info(),
            ctx.accounts.freeze_authority.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.cmt_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
        ],
        &[&ctx.accounts.policy.signer_seeds()],
    )?;

    Ok(())
}
