use anchor_lang::prelude::*;
#[error_code]
pub enum CustomError{
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Project not found")]
    ProjectNotFound,
    #[msg("Verifier not authorized")]
    VerifierNotWhitelisted
}