use anchor_lang::prelude::*;
use crate::state::Project;
use crate::state::Status;
#[derive(Accounts)]
pub struct RegisterProjectAccounts<'info> {
   
  #[account(
    init,
    payer=owner,
    seeds=[b"register_project"],
    space=Project::DISCRIMINATOR.len()+Project::INIT_SPACE,
    bump,
  )]
  pub project:Account<'info,Project>,
  #[account(mut)]
  pub owner:Signer<'info>,
pub system_program:Program<'info,System>,
}
impl <'info>RegisterProjectAccounts<'info>{
    pub fn register_project(&mut self,name:String,description:String,ipfs_hash:String)->Result<()>{
        let project_account=&mut self.project;
        let (_project_pda,project_bump)=
        Pubkey::find_program_address(&[b"register_project".as_ref()],&crate::ID);
        
        project_account.owner=self.owner.key();
        project_account.name=name;
        project_account.description=description;
        project_account.ipfs_hash=ipfs_hash;
        project_account.status=Status::NotVerified;
        project_account.verifier=[Pubkey::default();10];
        project_account.project_id=0;
        project_account.trust_score=0;
        project_account.bump=project_bump;

        Ok(())
    }
}