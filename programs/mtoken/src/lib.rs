pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

solana_program::declare_id!("mtokYxNhZEihbDq3r6LX22pLKnpuQvXV5kwhgCDCWw4");

#[program]
pub mod mtoken {
    use super::*;

    // mint_manager
    pub fn init_mint_manager(ctx: Context<InitMintManagerCtx>) -> Result<()> {
        mint_manager::init_mint_manager::handler(ctx)
    }

    pub fn update_mint_manager(
        ctx: Context<UpdateMintManagerCtx>,
        ix: UpdateMintManagerIx,
    ) -> Result<()> {
        mint_manager::update_mint_manager::handler(ctx, ix)
    }

    // ruleset
    pub fn init_ruleset(ctx: Context<InitRulesetV0Ctx>, ix: InitRulesetV0Ix) -> Result<()> {
        ruleset::init_ruleset_v0::handler(ctx, ix)
    }

    pub fn update_ruleset(ctx: Context<UpdateRulesetV0Ctx>, ix: UpdateRulesetV0Ix) -> Result<()> {
        ruleset::update_ruleset_v0::handler(ctx, ix)
    }

    // token
    pub fn init_mint(ctx: Context<InitMintCtx>) -> Result<()> {
        token::init_mint::handler(ctx)
    }

    pub fn init_account(ctx: Context<InitAccountCtx>) -> Result<()> {
        token::init_account::handler(ctx)
    }

    pub fn approve(ctx: Context<ApproveCtx>) -> Result<()> {
        token::approve::handler(ctx)
    }

    pub fn revoke(ctx: Context<RevokeCtx>) -> Result<()> {
        token::revoke::handler(ctx)
    }

    pub fn burn(ctx: Context<BurnCtx>) -> Result<()> {
        token::burn::handler(ctx)
    }

    pub fn close(ctx: Context<CloseCtx>) -> Result<()> {
        token::close::handler(ctx)
    }

    pub fn transfer(ctx: Context<TransferCtx>) -> Result<()> {
        token::transfer::handler(ctx)
    }
}
