use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Verifier not whitelisted")]
    VerifierNotWhitelisted,
    #[msg("Nothing to withdraw")]
    NothingToWithdraw,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Math overflow")]
    MathOverflow,
}
