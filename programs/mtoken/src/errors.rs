use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid collector address")]
    InvalidCollector,
    #[msg("Invalid authority address")]
    InvalidAuthority,
    #[msg("Invalid mint manager")]
    InvalidMintManager,
    #[msg("Invalid holder token account")]
    InvalidHolderTokenAccount,
    #[msg("Invalid target token account")]
    InvalidTargetTokenAccount,
    #[msg("Invalid token account to close")]
    InvalidCloseTokenAccount,
    #[msg("Invalid ruleset")]
    InvalidRuleset,
    #[msg("Invalid pre transfer instruction")]
    InvalidPreTransferInstruction,
    #[msg("Invalid post transfer instruction")]
    InvalidPostTransferInstruction,
    #[msg("Disallowed program included in transfer")]
    ProgramDisallowed,
    #[msg("Program not allowed in allowed programs to transfer")]
    ProgramNotAllowed,
    #[msg("Unknown account found in instruction")]
    UnknownAccount,
    #[msg("Account not found in instruction")]
    AccountNotFound,
}
