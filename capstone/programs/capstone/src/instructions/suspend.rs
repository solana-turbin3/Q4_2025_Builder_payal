use anchor_lang::prelude::*;
use crate::state::Project;
use crate::state::verifier;
use crate::state::Config;
#[derive(Accounts)]
pub struct SuspendProjectAccounts<'info> {
    #[account(
        mut,
        seeds = [b"project", project.owner.as_ref(), project.name.as_bytes()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(mut)]
    pub verifier: Signer<'info>,
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

}

// impl<'info>SuspendProjectAccounts<'info>{
//     pub fn suspend_project(&mut self)->Result<()>{
//         let project_account=&mut self.project;
//         let verifier_account=&self.verifier;
//         let config_account=&self.config;

//         // Check if the verifier is authorized
//         let is_authorized=verifier::is_authorized_verifier(
//             verifier_account.key(),
//             &config_account.authorized_verifiers
//         );

//         require!(
//             is_authorized,
//             CustomError::UnauthorizedVerifier
//         );

//         project_account.is_suspended=true;

//         Ok(())
//     }
// }