use std::cmp::{max, min};

use crate::errors::MTokenErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{Mint, TokenAccount};
use json_rules_engine::{Rule, Status};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_program::{
    program_option::COption, serialize_utils::read_u16,
    sysvar::instructions::load_instruction_at_checked,
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
        msg!("fact: {}", fact);
        msg!("json_rule: {}", self.json_rule);
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

fn to_option_str(c_option: COption<Pubkey>) -> Option<String> {
    match c_option {
        COption::Some(pubkey) => Some(pubkey.to_string()),
        COption::None => None,
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TokenAccountCtx {
    pub owner: String,
    pub amount: u64,
    pub delegate: Option<String>,
    pub delegated_amount: u64,
}

impl From<Box<Account<'_, TokenAccount>>> for TokenAccountCtx {
    fn from(account: Box<Account<'_, TokenAccount>>) -> Self {
        Self {
            owner: account.owner.to_string(),
            amount: account.amount,
            delegate: to_option_str(account.delegate),
            delegated_amount: account.delegated_amount,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MintAccountCtx {
    pub mint_authority: Option<String>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<String>,
}

impl From<Box<Account<'_, Mint>>> for MintAccountCtx {
    fn from(mint: Box<Account<'_, Mint>>) -> Self {
        MintAccountCtx {
            mint_authority: to_option_str(mint.mint_authority),
            supply: mint.supply,
            decimals: mint.decimals,
            is_initialized: mint.is_initialized,
            freeze_authority: to_option_str(mint.freeze_authority),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MintStateCtx {
    pub version: u8,
    pub policy: String,
    pub locked_by: Option<String>,
    pub last_approved_at: i64,
    pub last_transfered_at: i64,

    pub derived_cooldown: i64,
    pub derived_now: i64,
}

impl From<MintState> for MintStateCtx {
    fn from(mint_state: MintState) -> Self {
        let now = match Clock::get() {
            Ok(clock) => clock.unix_timestamp,
            Err(_) => 0, // use 0 as the default when Clock is not available, usually in test
        };

        MintStateCtx {
            version: mint_state.version,
            policy: mint_state.policy.to_string(),
            locked_by: mint_state.locked_by.map(|x| x.to_string()),
            last_approved_at: mint_state.last_approved_at,
            last_transfered_at: mint_state.last_transfered_at,

            derived_cooldown: min(
                max(0, now - mint_state.last_approved_at),
                max(0, now - mint_state.last_transfered_at),
            ),
            derived_now: now,
        }
    }
}

pub fn get_program_ids_from_instructions(ixs: &AccountInfo<'_>) -> Result<Vec<String>> {
    let instruction_sysvar = ixs.try_borrow_data()?;
    let mut current: usize = 0;
    let num_instructions =
        read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");
    let mut program_ids = Vec::<String>::new();
    for i in 0..num_instructions {
        let ix = load_instruction_at_checked(i.into(), ixs).expect("Failed to get instruction");
        program_ids.push(ix.program_id.to_string());
    }
    Ok(program_ids)
}

#[derive(Default, Serialize)]
pub struct ActionCtx {
    pub action: String,
    pub program_ids: Vec<String>,
    pub mint: String,
    pub mint_state: MintStateCtx,
    pub mint_account: Option<MintAccountCtx>,
    pub payer: Option<String>,
    pub from: Option<String>,
    pub from_account: Option<TokenAccountCtx>,
    pub to: Option<String>,
    pub to_account: Option<TokenAccountCtx>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy_fixture() -> Policy {
        Policy {
            version: 0,
            bump: [0; 1],
            update_authority: Pubkey::new_unique(),
            update_authority_nonce: [0; 1],
            json_rule:r#"{"conditions":{"and":[{"field":"action","operator":"string_not_equals","value":""}]},"events":[]}"#.to_string(),
        }
    }

    fn action_ctx_fixture() -> ActionCtx {
        ActionCtx {
            action: "transfer".to_string(),
            program_ids: vec![],
            mint: Pubkey::new_unique().to_string(),
            mint_state: MintState::default().into(),
            mint_account: None,
            payer: None,
            from: None,
            from_account: None,
            to: None,
            to_account: None,
        }
    }

    #[test]
    fn test_policy_validation() {
        let mut policy = policy_fixture();

        policy.json_rule = r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#.replace(
            "PLACEHOLDER",
            &(0..10).map(|_| format!("\"{}\"", Pubkey::new_unique().to_string())).collect::<Vec<String>>().join(","),
        );
        assert!(policy.valid().is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#.replace(
            "PLACEHOLDER",
            &(0..20).map(|_| format!("\"{}\"", Pubkey::new_unique().to_string())).collect::<Vec<String>>().join(","),
        );
        assert!(policy.valid().is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#.replace(
            "PLACEHOLDER",
            &(0..100).map(|_| format!("\"{}\"", Pubkey::new_unique().to_string())).collect::<Vec<String>>().join(","),
        );
        assert!(policy.valid().is_err());
    }

    #[test]
    fn test_policy_pass_all() {
        let policy = policy_fixture();
        let action_ctx = action_ctx_fixture();

        assert!(policy.valid().is_ok());
        assert!(policy.matches(action_ctx).is_ok());
    }

    #[test]
    fn test_policy_program_ids_single_allowlist() {
        let program_id = Pubkey::new_unique().to_string();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"and":[{"field":"program_ids","operator":"string_contains","value":"PLACEHOLDER"}]},"events":[]}
        "#.replace("PLACEHOLDER", &program_id);

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![program_id];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(action_ctx).is_err());
    }

    #[test]
    fn test_policy_program_ids_multiple_allowlist() {
        let allowed_program_ids = [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ];
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"field":"program_ids","operator":"string_is_subset","value":[PLACEHOLDER]},"events":[]}
        "#.replace(
            "PLACEHOLDER",
            &allowed_program_ids.clone().map(|x| format!("\"{}\"", x)).join(","),
        );
        assert!(policy.valid().is_ok());

        // ok with just allowed_program_ids[0]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![allowed_program_ids[0].clone()];
        assert!(policy.matches(action_ctx).is_ok());

        // ok with just allowed_program_ids[1]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![allowed_program_ids[1].clone()];
        assert!(policy.matches(action_ctx).is_ok());

        // not ok with some other program_id
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.matches(action_ctx).is_err());

        // not ok with some other program_id and allowed_program_ids[0]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![
            Pubkey::new_unique().to_string(),
            allowed_program_ids.clone()[0].clone(),
        ];
        assert!(policy.matches(action_ctx).is_err());
    }
}
