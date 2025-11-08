use anchor_lang::prelude::*;
use crate::state::VerifierRegistry;
#[derive(Accounts)]
pub struct AddVerifierAccounts<'info> {
    #[account(
        mut,
        seeds = [b"verifier"],
        bump = verifier_registry.bump
    )]
    pub verifier_registry: Account<'info, VerifierRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,
}

impl<'info> AddVerifierAccounts<'info> {
    pub fn add(&mut self, new_verifier: Pubkey) -> Result<()> {
        require_keys_eq!(
            self.verifier_registry.admin,
            self.admin.key(),
            // CustomError::Unauthorized
        );

        self.verifier_registry.verifier.push(new_verifier);
        msg!("Verifier added: {}", new_verifier);
        Ok(())
    }
}
