use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::program::invoke;
use solana_program::serialize_utils::read_u16;
use solana_program::system_instruction::create_account;
use solana_program::sysvar::instructions::load_instruction_at_checked;
use solana_program::sysvar::{self};
use std::collections::HashSet;

pub const PRE_TRANSFER_DISCRIMINATOR: [u8; 8] = [158, 85, 53, 202, 155, 118, 19, 228];

#[derive(Accounts)]
pub struct PreTransferCtx<'info> {
    /// CHECK: Checked by the transfer instruction
    #[account(mut)]
    account_balances: UncheckedAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
    // remaining_accounts
}

pub fn handler(ctx: Context<PreTransferCtx>) -> Result<()> {
    let instructions_account_info = ctx.accounts.instructions.to_account_info();
    let instruction_sysvar = instructions_account_info.try_borrow_data()?;
    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");

    let mut all_addresses = HashSet::new();
    for i in 0..num_instructions {
        let ix = load_instruction_at_checked(i.into(), &instructions_account_info)
            .expect("Failed to get instruction");
        for account in ix.accounts {
            all_addresses.insert(account.pubkey);
        }
    }

    let mut start_balances = Vec::new();
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    while let Some(account) = remaining_accounts.next() {
        if !all_addresses.remove(account.key) {
            return Err(error!(ErrorCode::UnknownAccount));
        }
        start_balances.push(AccountBalance {
            address: account.key(),
            size: if let Ok(data) = account.data.try_borrow() {
                data.len().try_into().expect("Conversion error")
            } else {
                0
            },
            balance: **account.lamports.borrow(),
            mint: Pubkey::default(),
        });

        if *account.owner == spl_token::id() {
            if let Ok(token_account) = Account::<TokenAccount>::try_from(&account) {
                start_balances.push(AccountBalance {
                    address: account.key(),
                    size: 0,
                    balance: token_account.amount,
                    mint: token_account.mint,
                });
            }
        }
    }
    if all_addresses.len() > 0 {
        return Err(error!(ErrorCode::AccountNotFound));
    }

    let space = 8 + std::mem::size_of_val(&start_balances) + 8;
    invoke(
        &create_account(
            ctx.accounts.payer.key,
            ctx.accounts.account_balances.key,
            Rent::get()?.minimum_balance(space),
            space.try_into().expect("Error allocating space"),
            ctx.program_id,
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.account_balances.to_account_info(),
        ],
    )?;

    // skip discriminator check
    let account_balances_account =
        &mut Account::<AccountBalances>::try_from_unchecked(&ctx.accounts.account_balances)?;
    account_balances_account.balances = start_balances;
    Ok(())
}
