use crate::action::*;
use crate::errors::OCPErrorCode;
use crate::id;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::metadata::MetadataAccount;
use anchor_spl::token::{Mint, Token, TokenAccount};
use community_managed_token::instruction::create_migrate_authority_instruction;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct MigrateToMplCtx<'info> {
    #[account(constraint = policy.to_account_info().owner.eq(&id()) @ OCPErrorCode::InvalidPolicyMintAssociation)]
    policy: Box<Account<'info, Policy>>,
    /// CHECK: checked in the mint.freeze_authority and mint.mint_authority constraints
    freeze_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = mint_state.mint == mint.key() @ OCPErrorCode::InvalidMint,
        constraint = mint_state.locked_by.is_none() @ OCPErrorCode::MintStateLocked,
        constraint = mint.decimals == 0 @ OCPErrorCode::InvalidMint, // nft
        constraint = mint.supply == 1 @ OCPErrorCode::InvalidMint, // nft
        constraint = mint.freeze_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = mint.mint_authority == COption::Some(freeze_authority.key()) @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = mint_state.policy == policy.key() @ OCPErrorCode::InvalidPolicyMintAssociation,
        constraint = policy.get_freeze_authority(policy.key()) == freeze_authority.key() @ OCPErrorCode::InvalidPolicyMintAssociation,
    )]
    mint: Box<Account<'info, Mint>>,
    /// CHECK: going to check in action ctx
    #[account(
        mut,
        seeds = [b"metadata", anchor_spl::metadata::Metadata::id().as_ref(), mint.key().as_ref()],
        seeds::program = anchor_spl::metadata::Metadata::id(),
        bump,
    )]
    metadata: Box<Account<'info, MetadataAccount>>,
    #[account(
        mut,
        close = from,
        seeds = [MintState::SEED.as_bytes(), mint.key().as_ref()],
        bump,
    )]
    mint_state: Box<Account<'info, MintState>>,
    #[account(mut)]
    from: Signer<'info>, // this is the update_authority of the metadata account
    #[account(
        mut,
        constraint = from_account.mint == mint.key() @ OCPErrorCode::InvalidTokenAccount,
        constraint = from_account.amount == 1 @ OCPErrorCode::InvalidTokenAccount,
        // for from_account.owner, we don't need to check the owner of the token account because
        // this migration is triggered by the update_authority (i.e. the "from" account)
    )]
    from_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: going to create this account in cpi
    #[account(mut)]
    edition: UncheckedAccount<'info>,
    /// CHECK: going to create this account in cpi
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    /// CHECK: checked in cpi
    #[account(address = community_managed_token::id())]
    cmt_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with mpl_token_metadata::id()
    #[account(address = anchor_spl::metadata::Metadata::id())]
    metadata_program: UncheckedAccount<'info>,
}

impl From<&mut MigrateToMplCtx<'_>> for ActionCtx {
    fn from(ctx: &mut MigrateToMplCtx) -> Self {
        let mut action_ctx = ActionCtx {
            action: "migrate_to_mpl".to_string(),
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

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, MigrateToMplCtx<'info>>) -> Result<()> {
    let action_ctx: ActionCtx = ctx.accounts.into();
    ctx.accounts.policy.matches(&action_ctx)?;

    // We need to verify that the metadata account's update authority is the same as the from account
    // Also, we already verified the mint -> metadata derivation via the to_metadata_ctx function
    if action_ctx.metadata.expect("metadata account should be available").update_authority != ctx.accounts.from.key().to_string() {
        return Err(OCPErrorCode::InvalidMetadataUpdateAuthority.into());
    }

    invoke_signed(
        &create_migrate_authority_instruction(
            &ctx.accounts.mint.key(),
            &ctx.accounts.policy.key(),
            &ctx.accounts.from.key(),
            &ctx.accounts.from.key(),
        )?,
        &[
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.policy.to_account_info(),
            ctx.accounts.freeze_authority.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.cmt_program.to_account_info(),
        ],
        &[&ctx.accounts.policy.signer_seeds()],
    )?;

    anchor_spl::token::thaw_account(anchor_lang::prelude::CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::ThawAccount {
            account: ctx.accounts.from_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        },
    ))?;

    invoke(
        &mpl_token_metadata::instruction::create_master_edition_v3(
            mpl_token_metadata::id(),
            ctx.accounts.edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.from.key(),
            ctx.accounts.from.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.from.key(),
            None,
        ),
        &[
            ctx.accounts.edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.from.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.metadata_program.to_account_info(),
        ],
    )?;

    Ok(())
}
