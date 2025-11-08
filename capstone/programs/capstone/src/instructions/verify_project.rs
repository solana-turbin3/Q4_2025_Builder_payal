use anchor_lang::prelude::*;
use crate::state::Config;
use crate::state::VerifierRegistry;
use crate::state::Project;
use crate::state::Attestation;
use crate::state::attestation;
use crate::state::verifier;
use crate::error::CustomError;
#[derive(Accounts)]
pub struct VerifyProjectAccounts<'info> {
    #[account(
        mut,
        seeds=[b"config"],
        bump=config.bump

    )]
    pub config: Account<'info, Config>,
   #[account(
        mut,
        seeds = [b"verifier"],
        bump = verifier_registry.bump
    )]
    pub verifier_registry: Account<'info, VerifierRegistry>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(
        init,
        payer=verifier,
        seeds=[b"Attestation",project.key().as_ref(),verifier.key().as_ref()],
        space=Attestation::DISCRIMINATOR.len()+Attestation::INIT_SPACE,
        bump,
    )]
    pub attestation: Account<'info, Attestation>,
    #[account(mut)]
    pub verifier: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info>VerifyProjectAccounts<'info>{
    pub fn verifier_project(&mut self,ipfs_hash:String,is_valid:bool)-> Result<()>  {
        let registry= &self.verifier_registry;
        let project=&mut self.project;
        let attestation=&mut self.attestation;

        require!(
            registry.verifier.contains(&self.verifier.key()),
             CustomError::VerifierNotWhitelisted
        );
        let (attestation_pda,attestation_bump)=
        Pubkey::find_program_address(
            &[b"Attestation",project.key().as_ref(),self.verifier.key().as_ref()],
            &crate::ID,
        );

        attestation.project= project.key();
        attestation.verifier=self.verifier.key();
        attestation.ipfs_hash=ipfs_hash;
        attestation.is_valid=is_valid;
        attestation.timestamp= Clock::get()?.unix_timestamp;
        attestation.bump=attestation_bump;

        if is_valid{
            project.trust_score=project.trust_score.saturating_add(10);

        }
        else{
            project.trust_score=project.trust_score.saturating_add(5);
        }
        Ok(())
    }
}
