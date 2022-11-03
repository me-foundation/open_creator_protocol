use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::FreezeAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::ThawAccount;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use solana_program::serialize_utils::read_u16;
use solana_program::sysvar;
use solana_program::sysvar::instructions::load_instruction_at_checked;
use std::cmp;
use std::collections::HashSet;

#[derive(Accounts)]
pub struct TransferCtx<'info> {
    #[account(constraint = mint.key() == mint_manager.mint @ ErrorCode::InvalidMintManager)]
    mint_manager: Box<Account<'info, MintManager>>,
    #[account(constraint = ruleset.key() == mint_manager.ruleset @ ErrorCode::InvalidRuleset)]
    ruleset: Account<'info, Ruleset>,
    mint: Box<Account<'info, Mint>>,

    #[account(mut, constraint =
        from.owner == holder.key()
        && from.mint == mint.key()
        && from.amount == 1 @ ErrorCode::InvalidHolderTokenAccount)]
    from: Account<'info, TokenAccount>,
    #[account(mut)]
    to: Account<'info, TokenAccount>,

    holder: Signer<'info>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferCtx>) -> Result<()> {
    let ruleset = &ctx.accounts.ruleset;
    let instructions = &ctx.accounts.instructions;
    let to = &ctx.accounts.to;
    let mint_manager_readonly = &ctx.accounts.mint_manager;
    assert_ruleset(ruleset, mint_manager_readonly, instructions, to)?;

    let mint = ctx.accounts.mint.key();
    let mint_manager_seeds = &[
        MINT_MANAGER_SEED.as_bytes(),
        mint.as_ref(),
        &[ctx.accounts.mint_manager.bump],
    ];
    let mint_manager_signer = &[&mint_manager_seeds[..]];

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.from.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::thaw_account(cpi_context)?;

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.holder.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, 1)?;

    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.to.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.mint_manager.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(mint_manager_signer);
    token::freeze_account(cpi_context)?;

    let mint_manager = &mut ctx.accounts.mint_manager;
    mint_manager.last_transfered_at = Clock::get().unwrap().unix_timestamp;

    Ok(())
}

fn assert_ruleset<'info>(
    ruleset: &Account<Ruleset>,
    mint_manager: &Account<'info, MintManager>,
    instructions: &UncheckedAccount<'info>,
    to: &Account<'info, TokenAccount>,
) -> Result<()> {
    let instruction_sysvar = instructions.try_borrow_data()?;
    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");

    // check allowed / disallowed
    let mut allowed_programs = HashSet::new();
    for program_id in &ruleset.rule_allow_programs {
        allowed_programs.insert(program_id);
    }

    let mut disallowed_addresses = HashSet::new();
    for program_id in &ruleset.rule_deny_addresses {
        disallowed_addresses.insert(program_id);
    }

    for i in 0..num_instructions {
        let ix =
            load_instruction_at_checked(i.into(), instructions).expect("Failed to get instruction");

        for account in ix.accounts {
            if !disallowed_addresses.is_empty() && disallowed_addresses.contains(&account.pubkey) {
                return Err(error!(ErrorCode::ProgramDisallowed));
            }
        }

        if should_check_allowed_programs(ruleset, mint_manager, to)?
            && !allowed_programs.is_empty()
            && !allowed_programs.contains(&ix.program_id)
        {
            return Err(error!(ErrorCode::ProgramNotAllowed));
        }
    }

    Ok(())
}

fn should_check_allowed_programs<'info>(
    ruleset: &Account<Ruleset>,
    mint_manager: &Account<'info, MintManager>,
    to: &Account<'info, TokenAccount>,
) -> Result<bool> {
    // check if any of the exceptions are set
    if ruleset.rule_allow_programs_except_non_pda_owner == bool::default()
        && ruleset.rule_allow_programs_except_cooldown_seconds == u32::default()
    {
        return Ok(true);
    }

    // check non_pda_owner exception rule
    // if the to.owner is a pda, we need to follow up with allowed_programs checks
    let to_owner_pda = !to.owner.is_on_curve();
    if ruleset.rule_allow_programs_except_non_pda_owner && to_owner_pda {
        return Ok(true);
    }

    // check cooldown exception rule
    // if last_approved_at or last_transfered_at is still recent enough, we need to follow up with allowed_programs checks
    let now = Clock::get().unwrap().unix_timestamp;
    let cooldown = cmp::min(
        now - mint_manager.last_approved_at,
        now - mint_manager.last_transfered_at,
    );
    if ruleset.rule_allow_programs_except_cooldown_seconds > 0
        && cooldown < ruleset.rule_allow_programs_except_cooldown_seconds.into()
    {
        return Ok(true);
    }

    Ok(false)
}
