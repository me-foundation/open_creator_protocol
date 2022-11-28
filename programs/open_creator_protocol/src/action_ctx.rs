use crate::{errors::OCPErrorCode, state::MintState};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Metadata, TokenMetadataAccount},
};
use serde::{Deserialize, Serialize};
use solana_program::{
    instruction::Instruction, program_option::COption, serialize_utils::read_u16,
    sysvar::instructions::load_instruction_at_checked,
};
use std::cmp::{max, min};

#[derive(Default, Serialize)]
pub struct ActionCtx {
    pub action: String,
    pub program_ids: Vec<String>,
    pub mint: String,
    pub mint_state: MintStateCtx,
    pub mint_account: Option<MintAccountCtx>,
    pub metadata: Option<MetadataCtx>,
    pub payer: Option<String>,
    pub from: Option<String>,
    pub from_account: Option<TokenAccountCtx>,
    pub to: Option<String>,
    pub to_account: Option<TokenAccountCtx>,
    pub last_memo_signer: Option<String>,
    pub last_memo_data: Option<String>,
}

impl ActionCtx {
    fn parse_memo(&mut self, ix: Instruction) {
        if ix.program_id.to_string() != "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr" {
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
        let num_instructions =
            read_u16(&mut current, &instruction_sysvar).expect("Invalid instruction");
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
    // root level
    pub update_authority: String,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub collection_verified: bool,
    pub collection_key: String,

    // data
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<String>>,
    pub creators_verified: Option<Vec<bool>>,
}

pub fn to_metadata_ctx(mint: &Pubkey, metadata: &AccountInfo) -> Result<MetadataCtx> {
    if find_metadata_account(mint).0 != metadata.key() {
        return Err(OCPErrorCode::InvalidMetadata.into());
    }
    let parsed_metadata = Metadata::from_account_info(metadata)?;
    let collection = parsed_metadata.collection.as_ref();
    let creators = parsed_metadata.data.creators.as_ref();
    Ok(MetadataCtx {
        update_authority: parsed_metadata.update_authority.to_string(),
        primary_sale_happened: parsed_metadata.primary_sale_happened,
        is_mutable: parsed_metadata.is_mutable,
        collection_verified: collection.map(|c| c.verified).unwrap_or(false),
        collection_key: collection
            .map(|c| c.key.to_string())
            .unwrap_or_else(|| "".to_owned()),
        name: parsed_metadata.data.name,
        uri: parsed_metadata.data.uri,
        symbol: parsed_metadata.data.symbol,
        seller_fee_basis_points: parsed_metadata.data.seller_fee_basis_points,
        creators: creators.map(|creators| {
            creators
                .iter()
                .map(|c| c.address.to_string())
                .collect::<Vec<String>>()
        }),
        creators_verified: creators
            .map(|creators| creators.iter().map(|c| c.verified).collect::<Vec<bool>>()),
    })
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

            derived_cooldown: min(
                max(0, now - mint_state.last_approved_at),
                max(0, now - mint_state.last_transferred_at),
            ),
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
            authority: Pubkey::new_unique(),
            uuid: Pubkey::new_unique(),
            json_rule:r#"{"conditions":{"and":[{"field":"action","operator":"string_not_equals","value":""}]},"events":[]}"#.to_string(),
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
            payer: None,
            from: None,
            from_account: None,
            to: None,
            to_account: None,
        }
    }

    fn metadata_ctx_fixture() -> MetadataCtx {
        MetadataCtx {
            name: "Test".to_string(),
            uri: "https://test.com".to_string(),
            symbol: "TEST".to_string(),
            seller_fee_basis_points: 500,
            update_authority: Pubkey::new_unique().to_string(),
            primary_sale_happened: true,
            is_mutable: true,
            creators: Some(
                [
                    Pubkey::new_unique().to_string(),
                    Pubkey::new_unique().to_string(),
                ]
                .to_vec(),
            ),
            creators_verified: Some(vec![true, false]),
            collection_verified: true,
            collection_key: Pubkey::new_unique().to_string(),
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
            &(0..18).map(|_| format!("\"{}\"", Pubkey::new_unique().to_string())).collect::<Vec<String>>().join(","),
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
        assert!(policy.matches(&action_ctx).is_ok());
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
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.program_ids = vec![Pubkey::new_unique().to_string()];
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
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
        action_ctx.program_ids = vec![
            Pubkey::new_unique().to_string(),
            allowed_program_ids.clone()[0].clone(),
        ];
        assert!(policy.matches(&action_ctx).is_err());
    }

    #[test]
    fn test_policy_with_metadata_policy() {
        let mut action_ctx = action_ctx_fixture();
        let creators = [
            Pubkey::new_unique().to_string(),
            Pubkey::new_unique().to_string(),
        ];
        let mut metadata = metadata_ctx_fixture();
        metadata.creators = Some(creators.clone().to_vec());
        action_ctx.metadata = Some(metadata);
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"field":"metadata/creators","operator":"string_is_subset","value":[PLACEHOLDER]},"events":[]}
        "#.replace(
            "PLACEHOLDER",
            &creators.clone().map(|x| format!("\"{}\"", x)).join(","),
        );
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
        policy.json_rule = r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":"(frozen)"},"events":[]}
        "#.into();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":""},"events":[]}
        "#.into();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"field":"metadata/name","operator":"string_has_substring","value":"SFT"},"events":[]}
        "#.into();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
    }

    #[test]
    fn test_policy_with_derived_datetime() {
        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":90},"events":[]}
        "#.to_string();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        action_ctx.action = "transfer".to_string();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":90}, {"field":"action","operator":"string_equals","value":"transfer"}]},"events":[]}
        "#.to_string();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        action_ctx.action = "transfer".to_string();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_timestamp","operator":"int_greater_than","value":110}, {"field":"action","operator":"string_equals","value":"transfer"}]},"events":[]}
        "#.to_string();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = 100.into();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_hour","operator":"int_in","value":[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}]},"events":[]}
        "#.to_string();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_ok());

        let mut action_ctx = action_ctx_fixture();
        action_ctx.mint_state.derived_datetime = (100 + 3600 * 12).into();
        let mut policy = policy_fixture();
        policy.json_rule = r#"
          {"conditions":{"and": [{"field":"mint_state/derived_datetime/utc_hour","operator":"int_in","value":[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}]},"events":[]}
        "#.to_string();
        assert!(policy.valid().is_ok());
        assert!(policy.matches(&action_ctx).is_err());
    }
}
