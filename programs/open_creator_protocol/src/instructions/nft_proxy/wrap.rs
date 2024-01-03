use crate::action::*;
use crate::errors::OCPErrorCode;
use crate::id;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::metadata::MetadataAccount;
use anchor_spl::token::set_authority;
use anchor_spl::token::{Mint, Token};
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct WrapCtx<'info> {
    #[account(
        constraint = policy.to_account_info().owner.eq(&id()) @ OCPErrorCode::InvalidPolicyMintAssociation,
    )]
    policy: Box<Account<'info, Policy>>,
    freeze_authority: Signer<'info>,
    mint_authority: Signer<'info>,
    #[account(
        mut,
        constraint = mint.decimals == 0 @ OCPErrorCode::InvalidMint, // nft
        constraint = mint.supply == 0 @ OCPErrorCode::InvalidMint, // nft
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidMint,
        constraint = mint.mint_authority == COption::Some(mint_authority.key()) @ OCPErrorCode::InvalidMint,
    )]
    mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"metadata", anchor_spl::metadata::Metadata::id().as_ref(), mint.key().as_ref()],
        seeds::program = anchor_spl::metadata::Metadata::id(),
        bump,
    )]
    metadata: Box<Account<'info, MetadataAccount>>,
    #[account(
        init,
        payer = from,
        seeds = [MintState::SEED.as_bytes(), mint.key().as_ref()],
        space = MintState::LEN,
        bump,
    )]
    mint_state: Box<Account<'info, MintState>>,
    #[account(mut)]
    from: Signer<'info>,
    /// CHECK: going to create this account in cpi
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

impl From<&mut WrapCtx<'_>> for ActionCtx {
    fn from(ctx: &mut WrapCtx) -> Self {
        let mut action_ctx = ActionCtx {
            action: "wrap".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            payer: None,
            from: Some(ctx.from.key().to_string()),
            to: None,
            mint: ctx.mint.key().to_string(),
            metadata: Some(ctx.metadata.clone().into()),
            mint_account: None,
            mint_state: ctx.mint_state.clone().into_inner().into(),
        };
        action_ctx
            .parse_instructions(&ctx.instructions)
            .expect("failed to parse sysvar instructions");
        action_ctx
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, WrapCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    let policy = &ctx.accounts.policy;
    ctx.accounts.policy.matches(&action_ctx)?;

    let mint_state = &mut ctx.accounts.mint_state;
    mint_state.bump = [ctx.bumps.mint_state];
    mint_state.policy = policy.key();
    mint_state.mint = ctx.accounts.mint.key();
    mint_state.version = 0;

    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::SetAuthority {
                current_authority: ctx.accounts.freeze_authority.to_account_info(),
                account_or_mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount,
        Some(policy.get_freeze_authority(policy.key())),
    )?;

    set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::SetAuthority {
                current_authority: ctx.accounts.mint_authority.to_account_info(),
                account_or_mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
        Some(policy.get_freeze_authority(policy.key())),
    )?;

    Ok(())
}
