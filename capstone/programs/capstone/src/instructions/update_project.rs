use anchor_lang::prelude::*;
use crate::state::Project;
use crate::error::CustomError;
#[derive(Accounts)]
pub struct UpdateProjectAccounts<'info> {
   #[account(
         mut,
         seeds = [b"project", 
         project.owner.as_ref(),
          &project.project_id.to_le_bytes()
],
         bump = project.bump
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

        Ok(())
    }
}