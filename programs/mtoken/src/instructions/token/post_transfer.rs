use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::serialize_utils::read_u16;
use solana_program::sysvar::instructions::load_instruction_at_checked;
use solana_program::sysvar::{self};
use std::collections::HashMap;
use std::collections::HashSet;

pub const POST_TRANSFER_DISCRIMINATOR: [u8; 8] = [195, 252, 43, 202, 149, 119, 175, 84];

#[derive(Accounts)]
pub struct PostTransferCtx<'info> {
    /// CHECK: Checked by the transfer instruction
    #[account(mut, close = collector)]
    account_balances: Box<Account<'info, AccountBalances>>,
    /// CHECK: This is not dangerous because it is recipient of payment
    #[account(mut)]
    collector: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because the ID is checked with instructions sysvar
    #[account(address = sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
    // remaining_accounts
}

pub fn handler(ctx: Context<PostTransferCtx>) -> Result<()> {
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

    let mut end_balances = HashMap::new();
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    while let Some(account) = remaining_accounts.next() {
        if !all_addresses.remove(account.key) {
            return Err(error!(ErrorCode::UnknownAccount));
        }
        end_balances.insert(
            [account.key(), Pubkey::default()],
            AccountBalance {
                address: account.key(),
                size: if let Ok(data) = account.data.try_borrow() {
                    data.len().try_into().expect("Conversion error")
                } else {
                    0
                },
                balance: **account.lamports.borrow(),
                mint: Pubkey::default(),
            },
        );

        if *account.owner == spl_token::id() {
            if let Ok(token_account) = Account::<TokenAccount>::try_from(&account) {
                end_balances.insert(
                    [account.key(), token_account.mint],
                    AccountBalance {
                        address: account.key(),
                        size: 0,
                        balance: token_account.amount,
                        mint: token_account.mint,
                    },
                );
            }
        }
    }
    if all_addresses.len() > 0 {
        return Err(error!(ErrorCode::AccountNotFound));
    }

    let mut balance_change_by_mint: HashMap<Pubkey, u64> = HashMap::new();
    for account_balance in &ctx.accounts.account_balances.balances {
        let end_balance = end_balances
            .get(&[account_balance.address, account_balance.mint])
            .expect("Expected to find balance");

        if end_balance.mint == Pubkey::default() {
            // saturating sub rent_exempt_minimum from start and end
            let rent_exempt_minimum = Rent::get()?
                .minimum_balance(account_balance.size.try_into().expect("Conversion error"));
            let diff = end_balance
                .balance
                .saturating_sub(rent_exempt_minimum)
                .saturating_sub(account_balance.balance.saturating_sub(rent_exempt_minimum));
            let v = if let Some(current_value) = balance_change_by_mint.get(&end_balance.mint) {
                *current_value
            } else {
                0
            };
            balance_change_by_mint
                .insert(end_balance.mint, v.checked_add(diff).expect("Add error"));
        } else {
            let diff = end_balance.balance.saturating_sub(account_balance.balance);
            let v = if let Some(current_value) = balance_change_by_mint.get(&end_balance.mint) {
                *current_value
            } else {
                0
            };
            balance_change_by_mint
                .insert(end_balance.mint, v.checked_add(diff).expect("Add error"));
        }
    }

    Ok(())
}
