use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::{Treasury, VerifierRegistry};
use crate::error::CustomError;

#[derive(Accounts)]
pub struct WithdrawVerifier<'info> {
    #[account(mut)]
    pub verifier: Signer<'info>,

    /// Treasury PDA (holds lamports)
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
       #[account(
        mut,
        seeds = [b"treasury_vault"],
        bump = treasury.vault_bump
    )]
    pub vault: SystemAccount<'info>,


    /// Verifier registry (contains per-verifier attestation counts and total)
    #[account(mut, seeds = [b"verifier"], bump = verifier_registry.bump)]
    pub verifier_registry: Account<'info, VerifierRegistry>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawVerifier<'info> {
    pub fn withdraw_verifier(&mut self) -> Result<()> {
        // find verifier index
        let pos = self
            .verifier_registry
            .verifier
            .iter()
            .position(|v| v == &self.verifier.key())
            .ok_or(CustomError::VerifierNotWhitelisted)?;

        let att_count = self.verifier_registry.attestation_counts[pos];
        require!(att_count > 0, CustomError::NothingToClaim);

        let total = self.verifier_registry.total_attestations;
        require!(total > 0, CustomError::NothingToClaim);

        let pool = self.treasury.verifier_pool;
        // pro rata calculation with u128 to avoid overflow
        let amount_u128 = (pool as u128)
            .checked_mul(att_count as u128)
            .ok_or(CustomError::MathOverflow)?
            .checked_div(total as u128)
            .ok_or(CustomError::MathOverflow)?;

        let amount = amount_u128 as u64;
        require!(amount > 0, CustomError::NothingToClaim);

        // transfer from treasury PDA to verifier
        let bump = self.treasury.vault_bump;
        let seeds = &[b"treasury_vault".as_ref(), &[bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.vault.to_account_info(),
                to: self.verifier.to_account_info(),
            },
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, amount)?;

        // bookkeeping: reduce pool and mark attestations claimed
        self.treasury.verifier_pool = self.treasury.verifier_pool.saturating_sub(amount);
        self.verifier_registry.total_attestations = self
            .verifier_registry
            .total_attestations
            .saturating_sub(att_count);
        self.verifier_registry.attestation_counts[pos] = 0;

        Ok(())
    }
}
