use crate::state::MintState;
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::MetadataAccount,
    token::{Mint, TokenAccount},
};
use serde::{Deserialize, Serialize};
use solana_program::{
    instruction::Instruction, program_option::COption, serialize_utils::read_u16, sysvar::instructions::load_instruction_at_checked,
};
use std::cmp::max;

#[derive(Default, Serialize)]
pub struct ActionCtx {
    pub action: String,
    pub program_ids: Vec<String>,
    pub mint: String,
    pub mint_state: MintStateCtx,
    pub mint_account: Option<MintAccountCtx>,
    pub metadata: Option<MetadataCtx>,
    pub payer: Option<String>,
    pub from: Option<String>, // owner of the from_account, and many action's initiator
    pub to: Option<String>,   // owner of the to_account
    pub last_memo_signer: Option<String>,
    pub last_memo_data: Option<String>,
}

impl ActionCtx {
    fn parse_memo(&mut self, ix: Instruction) {
        if ix.program_id != spl_memo::id() {
            return;
        }
        if ix.accounts.is_empty() {
            return;
        }
        self.last_memo_signer = match ix.accounts[0].is_signer {
            true => Some(ix.accounts[0].pubkey.to_string()),
            false => None,
        };
        self.last_memo_data = match String::from_utf8(ix.data) {
            Ok(s) => Some(s),
            Err(_) => None,
        };
    }

    pub fn parse_instructions(&mut self, ixs: &AccountInfo<'_>) -> Result<()> {
        let instruction_sysvar = ixs.try_borrow_data()?;
        let mut current: usize = 0;
        let num_instructions = read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");
        let mut program_ids = Vec::<String>::new();
        for i in 0..num_instructions {
            let ix = load_instruction_at_checked(i.into(), ixs).expect("Failed to get instruction");
            program_ids.push(ix.program_id.to_string());
            self.parse_memo(ix);
        }

        self.program_ids = program_ids;
        Ok(())
    }
}

fn to_option_str(c_option: COption<Pubkey>) -> Option<String> {
    match c_option {
        COption::Some(pubkey) => Some(pubkey.to_string()),
        COption::None => None,
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MetadataCtx {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub update_authority: String,
}

impl From<Box<Account<'_, MetadataAccount>>> for MetadataCtx {
    fn from(metadata: Box<Account<'_, MetadataAccount>>) -> Self {
        Self {
            name: metadata.data.name.clone(),
            symbol: metadata.data.symbol.clone(),
            uri: metadata.data.uri.clone(),
            seller_fee_basis_points: metadata.data.seller_fee_basis_points,
            update_authority: metadata.update_authority.to_string(),
        }
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
pub struct DatetimeCtx {
    pub utc_timestamp: i64,
    pub utc_hour: u8,
}

impl From<i64> for DatetimeCtx {
    fn from(secs: i64) -> Self {
        Self {
            utc_timestamp: secs,
            utc_hour: (secs / 3600 % 24) as u8,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MintStateCtx {
    pub version: u8,
    pub policy: String,
    pub locked_by: Option<String>,
    pub last_approved_at: i64,
    pub last_transferred_at: i64,
    pub transferred_count: u32,

    // derived from existing fields
    pub derived_cooldown: i64,
    pub derived_datetime: DatetimeCtx,
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
            last_transferred_at: mint_state.last_transferred_at,
            transferred_count: mint_state.transferred_count,

            derived_cooldown: (now - mint_state.last_approved_at).clamp(0, max(0, now - mint_state.last_transferred_at)),
            derived_datetime: now.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Policy;

    fn policy_fixture() -> Policy {
        Policy {
            version: 0,
            bump: [0; 1],
            uuid: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            dynamic_royalty: None,
            json_rule: Some(r#"{"conditions":{"and":[{"field":"action","operator":"string_not_equals","value":""}]},"events":[]}"#.to_string()),
        }
    }

    fn action_ctx_fixture() -> ActionCtx {
        ActionCtx {
            action: "transfer".to_string(),
            program_ids: vec![],
            last_memo_data: None,
            last_memo_signer: None,
            mint: Pubkey::new_unique().to_string(),
            mint_state: MintState::default().into(),
            mint_account: None,
            metadata: None,
            payer: Some(Pubkey::new_unique().to_string()),
            from: Some(Pubkey::new_unique().to_string()),
            to: Some(Pubkey::new_unique().to_string()),
        }
    }

    fn metadata_ctx_fixture() -> MetadataCtx {
        MetadataCtx {
            name: "Test".to_string(),
            uri: "https://test.com".to_string(),
            symbol: "TEST".to_string(),
            seller_fee_basis_points: 500,
            update_authority: Pubkey::new_unique().to_string(),
        }
    }

    #[test]
    fn test_policy_validation() {
        let mut policy = policy_fixture();

        policy.json_rule = Some(
            r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#
            .replace(
                "PLACEHOLDER",
                &(0..10)
                    .map(|_| format!("\"{}\"", Pubkey::new_unique()))
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        );
        assert!(policy.valid().is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#
            .replace(
                "PLACEHOLDER",
                &(0..18)
                    .map(|_| format!("\"{}\"", Pubkey::new_unique()))
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        );
        assert!(policy.valid().is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"not":{"field":"program_ids","operator":"string_does_not_contain_any","value":[PLACEHOLDER]}},"events":[]}
        "#
            .replace(
                "PLACEHOLDER",
                &(0..100)
                    .map(|_| format!("\"{}\"", Pubkey::new_unique()))
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        );
        assert!(policy.valid().is_err());
    }

    #[test]
    fn test_policy_pass_all() {
        let policy = policy_fixture();
        let action_ctx = action_ctx_fixture();

        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());
    }

    #[test]
    fn test_policy_program_ids_single_allowlist() {
        let program_id = Pubkey::new_unique().to_string();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"and":[{"field":"program_ids","operator":"string_contains","value":"PLACEHOLDER"}]},"events":[]}
        "#
            .replace("PLACEHOLDER", &program_id),
        );

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![program_id];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
    }

    #[test]
    fn test_policy_program_ids_denylist() {
        let program_id = Pubkey::new_unique().to_string();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"and":[{"field":"program_ids","operator":"string_does_not_contain_any","value":["PLACEHOLDER"]}]},"events":[]}
        "#
            .replace("PLACEHOLDER", &program_id),
        );
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![program_id];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let program_id = Pubkey::new_unique().to_string();
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#" {"conditions":{"and":[{"field":"program_ids","operator":"string_does_not_contain_any","value":[]}]},"events":[]} "#.to_owned());
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![program_id];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let program_id = Pubkey::new_unique().to_string();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#" {"conditions":{"and":[{"field":"program_ids","operator":"string_does_not_contain_any","value":[""]}]},"events":[]} "#.to_owned(),
        );
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![program_id];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());
    }

    #[test]
    fn test_policy_program_ids_multiple_allowlist() {
        let allowed_program_ids = [Pubkey::new_unique().to_string(), Pubkey::new_unique().to_string()];
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"field":"program_ids","operator":"string_is_subset","value":[PLACEHOLDER]},"events":[]}
        "#
            .replace("PLACEHOLDER", &allowed_program_ids.clone().map(|x| format!("\"{}\"", x)).join(",")),
        );
        assert!(policy.valid().is_ok());

        // ok with just allowed_program_ids[0]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![allowed_program_ids[0].clone()];
        assert!(policy.matches(&action_ctx).is_ok());

        // ok with just allowed_program_ids[1]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![allowed_program_ids[1].clone()];
        assert!(policy.matches(&action_ctx).is_ok());

        // not ok with some other program_id
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.matches(&action_ctx).is_err());

        // not ok with some other program_id and allowed_program_ids[0]
        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string(), allowed_program_ids.clone()[0].clone()];
        assert!(policy.matches(&action_ctx).is_err());
    }

    #[test]
    fn test_policy_with_metadata_policy() {
        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc FROZEN".to_owned();
        action_ctx.metadata = Some(metadata);
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"conditions":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"},"events":[]}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc".to_owned();
        action_ctx.metadata = Some(metadata);
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"events":[],"conditions":{"or":[{"field":"action","operator":"string_not_equals","value":"transfer"},{"and":[{"not":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"}},{"or":[{"field":"to","operator":"string_not_equals","value":"DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"},{"field":"metadata/name","operator":"string_has_substring","value":"WINNER"}]}]}]}}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc FROZEN".to_owned();
        action_ctx.metadata = Some(metadata);
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"events":[],"conditions":{"or":[{"field":"action","operator":"string_not_equals","value":"transfer"},{"and":[{"not":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"}},{"or":[{"field":"to","operator":"string_not_equals","value":"DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"},{"field":"metadata/name","operator":"string_has_substring","value":"WINNER"}]}]}]}}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());

        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc".to_owned();
        action_ctx.metadata = Some(metadata);
        action_ctx.to = Some("DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn".to_owned());
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"events":[],"conditions":{"or":[{"field":"action","operator":"string_not_equals","value":"transfer"},{"and":[{"not":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"}},{"or":[{"field":"to","operator":"string_not_equals","value":"DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"},{"field":"metadata/name","operator":"string_has_substring","value":"WINNER"}]}]}]}}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());

        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc WINNER".to_owned();
        action_ctx.metadata = Some(metadata);
        action_ctx.to = Some("DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn".to_owned());
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"events":[],"conditions":{"or":[{"field":"action","operator":"string_not_equals","value":"transfer"},{"and":[{"not":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"}},{"or":[{"field":"to","operator":"string_not_equals","value":"DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"},{"field":"metadata/name","operator":"string_has_substring","value":"WINNER"}]}]}]}}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "abc".to_owned();
        action_ctx.action = "approve".to_owned();
        action_ctx.metadata = Some(metadata);
        action_ctx.to = Some("DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn".to_owned());
        let mut policy = policy_fixture();
        policy.json_rule =
            Some(r#"{"events":[],"conditions":{"or":[{"field":"action","operator":"string_not_equals","value":"transfer"},{"and":[{"not":{"field":"metadata/name","operator":"string_has_substring","value":"FROZEN"}},{"or":[{"field":"to","operator":"string_not_equals","value":"DWuopEsTrg5qWMSMVT1hoiVTRQG9PkGJZSbXiKAxHYbn"},{"field":"metadata/name","operator":"string_has_substring","value":"WINNER"}]}]}]}}"#.to_owned());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());
    }

    #[test]
    fn test_policy_with_metadata_name_substring() {
        let mut action_ctx = action_ctx_fixture();
        let mut metadata = metadata_ctx_fixture();
        metadata.name = "NFT #1 (frozen)".to_string();
        action_ctx.metadata = Some(metadata);

        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":"(frozen)"},"events":[]}
        "#
            .into(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":""},"events":[]}
        "#
            .into(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":"SFT"},"events":[]}
        "#
            .into(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
    }

    #[test]
    fn test_policy_with_derived_datetime() {
        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":90},"events":[]}
        "#
            .to_string(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        action_ctx.action = "transfer".to_string();
        let mut policy = policy_fixture();
        policy.json_rule = Some(r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":90}, {"field":"action","operator":"string_equals","value":"transfer"}]},"events":[]}
        "#.to_string());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        action_ctx.action = "transfer".to_string();
        let mut policy = policy_fixture();
        policy.json_rule = Some(r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":110}, {"field":"action","operator":"string_equals","value":"transfer"}]},"events":[]}
        "#.to_string());
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_hour","operator":"int_in","value":[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}]},"events":[]}
        "#
            .to_string(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = (100 + 3600 * 12).into();
        let mut policy = policy_fixture();
        policy.json_rule = Some(
            r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_hour","operator":"int_in","value":[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}]},"events":[]}
        "#
            .to_string(),
        );
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
    }
}
