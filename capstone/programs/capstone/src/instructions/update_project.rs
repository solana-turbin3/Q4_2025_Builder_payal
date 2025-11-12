use anchor_lang::prelude::*;
use crate::state::{Project, Status};
use crate::error::CustomError;
#[derive(Accounts)]
pub struct UpdateProjectAccounts<'info> {
   #[account(
         mut,
         has_one=owner,
         seeds = [b"project", project.owner.as_ref(), project.id.as_bytes()],
        bump = project.bump,
   )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub owner: Signer<'info>,
}
impl<'info>UpdateProjectAccounts<'info>{
    pub fn update_project(&mut self,name:String,description:String,ipfs_hash:String)->Result<()>{
        let project_account=&mut self.project;

        require_keys_eq!(
            project_account.owner,
            self.owner.key(),
            CustomError::Unauthorized
        );

        project_account.name=name;
        project_account.description=description;
        project_account.ipfs_hash=ipfs_hash;
        project_account.status=Status::UnderReview;
       project_account.version = project_account.version.saturating_add(1);
         msg!("Project {} updated. New version: {}", project_account.id, project_account.version);
        Ok(())
    }
}