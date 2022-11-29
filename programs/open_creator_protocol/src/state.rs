use crate::{action::ActionCtx, errors::OCPErrorCode, royalty::DynamicRoyalty};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use json_rules_engine_fork::{Rule, Status};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[account]
#[derive(Default, Serialize, Deserialize)]
pub struct MintState {
    pub version: u8,
    pub bump: [u8; 1],
    pub mint: Pubkey,
    pub policy: Pubkey,
    pub locked_by: Option<Pubkey>,
    pub last_approved_at: i64,
    pub last_transferred_at: i64,
    pub transferred_count: u32,
}

impl MintState {
    pub const LEN: usize = 200;
    pub const SEED: &'static str = "mint_state";

    pub fn record_transfer(&mut self) {
        self.last_transferred_at = Clock::get().unwrap().unix_timestamp;
        self.transferred_count = self.transferred_count.checked_add(1).unwrap_or(u32::MAX);
    }
    pub fn record_approve(&mut self) {
        self.last_approved_at = Clock::get().unwrap().unix_timestamp;
    }
}

#[account]
#[derive(Default, Serialize, Deserialize)]
pub struct Policy {
    pub version: u8,
    pub bump: [u8; 1],
    pub uuid: Pubkey,
    pub authority: Pubkey,
    pub dynamic_royalty: Option<DynamicRoyalty>,
    pub json_rule: Option<String>,
}

impl Policy {
    pub const LEN: usize = Policy::JSON_RULE_MAX_LEN + 400 /* with padding */;
    pub const SEED: &'static str = "policy";
    pub const MANAGED_AUTHORITY: &'static str = "RULERZZDGsXqd9TeJu5ikLfbXzBFpoDPT8N3FHRhq1T";
    pub const JSON_RULE_MAX_LEN: usize = 1000;

    pub fn valid(&self) -> Result<()> {
        match &self.json_rule {
            Some(json_rule) => {
                if json_rule.len() > Policy::JSON_RULE_MAX_LEN {
                    return Err(OCPErrorCode::InvalidPolicyCreation.into());
                }
                serde_json::from_str::<Rule>(json_rule).expect("json_rule should be valid");
            }
            None => {}
        }
        match &self.dynamic_royalty {
            Some(dynamic_royalty) => {
                dynamic_royalty.valid()?;
            }
            None => {}
        }
        Ok(())
    }

    pub fn is_managed(&self) -> bool {
        self.authority.to_string() == Policy::MANAGED_AUTHORITY
    }

    pub fn matches(&self, ctx: &ActionCtx) -> Result<()> {
        match &self.json_rule {
            Some(json_rule) => {
                if json_rule.is_empty() {
                    return Ok(());
                }
                let rule: Rule = serde_json::from_str::<Rule>(json_rule).expect("json_rule should be valid");
                let fact: &Value = &serde_json::to_value::<&ActionCtx>(ctx).expect("action_ctx should be serializable");
                let result = rule.check_value(fact);
                if result.condition_result.status != Status::Met {
                    msg!("Policy does not match: {}", result.condition_result.name);
                    msg!("fact: {}", fact);
                    msg!("json_rule: {}", json_rule);
                    return Err(OCPErrorCode::InvalidPolicyEvaluation.into());
                }
            }
            None => {}
        }

        Ok(())
    }

    pub fn signer_seeds(&self) -> [&[u8]; 3] {
        [Policy::SEED.as_bytes(), self.uuid.as_ref(), &self.bump]
    }

    pub fn get_freeze_authority(&self, upstream_authority: Pubkey) -> Pubkey {
        let (freeze_authority, _) = Pubkey::find_program_address(&[upstream_authority.as_ref()], &community_managed_token::id());
        freeze_authority
    }
}
