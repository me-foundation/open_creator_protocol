use crate::errors::MTokenErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{Mint, TokenAccount};
use json_rules_engine::{Rule, Status};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::{
    serialize_utils::read_u16, sysvar::instructions::load_instruction_at_checked,
};

#[account]
#[derive(Default, Serialize, Deserialize)]
pub struct MintState {
    pub version: u8,
    pub bump: [u8; 1],
    pub mint: Pubkey,
    pub policy: Pubkey,
    pub locked_by: Option<Pubkey>,
    pub last_approved_at: i64,
    pub last_transfered_at: i64,
}

#[account]
#[derive(Default, Serialize, Deserialize)]
pub struct Policy {
    pub version: u8,
    pub bump: [u8; 1],
    pub update_authority: Pubkey,
    pub update_authority_nonce: [u8; 1],
    pub json_rule: String,
}

impl MintState {
    pub const LEN: usize = 200;
    pub const SEED: &'static str = "mint_state";

    pub fn assert_unlocked(&self) -> Result<()> {
        if self.locked_by.is_some() {
            return Err(MTokenErrorCode::MintStateLocked.into());
        }
        Ok(())
    }
}

impl Policy {
    pub const LEN: usize = Policy::JSON_RULE_MAX_LEN + 200 /* with padding */;
    pub const SEED: &'static str = "policy";
    pub const MANAGED_AUTHORITY: &'static str = "RULERZZDGsXqd9TeJu5ikLfbXzBFpoDPT8N3FHRhq1T";
    pub const JSON_RULE_MAX_LEN: usize = 2000;

    pub fn valid(&self) -> Result<()> {
        if self.json_rule.len() > Policy::JSON_RULE_MAX_LEN {
            return Err(MTokenErrorCode::InvalidPolicyCreation.into());
        }
        // make sure the rule is valid
        serde_json::from_str::<Rule>(&self.json_rule).unwrap();
        Ok(())
    }

    pub fn matches(&self, ctx: ActionCtx) -> Result<()> {
        if self.json_rule.is_empty() {
            return Ok(());
        }

        let rule: Rule = serde_json::from_str::<Rule>(&self.json_rule).unwrap();
        let fact: &Value = &serde_json::to_value::<ActionCtx>(ctx).unwrap();
        let result = rule.check_value(fact);
        if result.condition_result.status != Status::Met {
            msg!("Policy does not match: {}", result.condition_result.name);
            return Err(MTokenErrorCode::InvalidPolicyEvaluation.into());
        }
        Ok(())
    }

    pub fn signer_seeds(&self) -> [&[u8]; 4] {
        [
            Policy::SEED.as_bytes(),
            self.update_authority.as_ref(),
            &self.update_authority_nonce,
            &self.bump,
        ]
    }

    pub fn get_freeze_authority(&self, upstream_authority: Pubkey) -> Pubkey {
        let (freeze_authority, _) = Pubkey::find_program_address(
            &[upstream_authority.as_ref()],
            &community_managed_token::id(),
        );
        freeze_authority
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TokenAccountCtx {
    pub owner: Pubkey,
    pub amount: u64,
    pub delegate: Option<Pubkey>,
    pub delegated_amount: u64,
}

impl From<Box<Account<'_, TokenAccount>>> for TokenAccountCtx {
    fn from(account: Box<Account<'_, TokenAccount>>) -> Self {
        Self {
            owner: account.owner,
            amount: account.amount,
            delegate: account.delegate.into(),
            delegated_amount: account.delegated_amount,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MintAccountCtx {
    pub mint_authority: Option<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

impl From<Box<Account<'_, Mint>>> for MintAccountCtx {
    fn from(mint: Box<Account<'_, Mint>>) -> Self {
        MintAccountCtx {
            mint_authority: mint.mint_authority.into(),
            supply: mint.supply,
            decimals: mint.decimals,
            is_initialized: mint.is_initialized,
            freeze_authority: mint.freeze_authority.into(),
        }
    }
}

pub fn get_program_ids_from_instructions(ixs: &AccountInfo<'_>) -> Result<Vec<Pubkey>> {
    let instruction_sysvar = ixs.try_borrow_data()?;
    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");
    let mut program_ids = Vec::<Pubkey>::new();
    for i in 0..num_instructions {
        let ix = load_instruction_at_checked(i.into(), ixs).expect("Failed to get instruction");
        program_ids.push(ix.program_id);
    }
    Ok(program_ids)
}

#[derive(Default, Serialize, Deserialize)]
pub struct ActionCtx {
    pub action: String,
    pub program_ids: Vec<Pubkey>,
    pub mint: Pubkey,
    pub mint_state: MintState,
    pub mint_account: Option<MintAccountCtx>,
    pub payer: Option<Pubkey>,
    pub from: Option<Pubkey>,
    pub from_account: Option<TokenAccountCtx>,
    pub to: Option<Pubkey>,
    pub to_account: Option<TokenAccountCtx>,
}
