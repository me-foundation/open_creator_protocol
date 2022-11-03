use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub const RULESET_AUTHORITY: &str = "RULERZZDGsXqd9TeJu5ikLfbXzBFpoDPT8N3FHRhq1T";

pub const MINT_MANAGER_SEED: &str = "mint-manager";
pub const MINT_MANAGER_SIZE: usize = 8 + 1 + 1 + 32 * 3 + 8 * 2 + 200;
#[account]
pub struct MintManager {
    pub bump: u8,
    pub version: u8,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub ruleset: Pubkey,
    pub last_approved_at: i64,
    pub last_transfered_at: i64,
}

pub const RULESET_SEED: &str = "ruleset";
pub const RULESET_SIZE: usize = 8 + 1 + 1 + 32 * 10 + 32 * 2 + 1 + 32 * 20 + 4 + 200;
#[account]
pub struct Ruleset {
    pub bump: u8,
    pub version: u8,
    pub name: String,
    pub authority: Pubkey,
    pub collector: Pubkey,
    pub rule_allow_programs_except_non_pda_owner: bool,
    pub rule_allow_programs_except_cooldown_seconds: u32,
    pub rule_deny_addresses: Vec<Pubkey>,
    pub rule_allow_programs: Vec<Pubkey>,
}
