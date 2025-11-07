use anchor_lang::prelude::*;
use anchor_lang::solana_program::config;
use crate::state::Project;
use crate::state::Status;
use crate::state::Config;
#[derive(Accounts)]
pub struct RegisterProjectAccounts<'info> {
   #[account(
    mut,
    seeds=[b"config"],
    bump=config.bump,
   )]
   pub config:Account<'info,Config>,
   #[account(
    mut,
    seeds=[b"treasury"],
    bump,
   )]
    pub treasury:AccountInfo<'info>,
  #[account(
    init,
    payer=owner,
    seeds=[b"register_project",owner.key().as_ref()],
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
        let config_account=&mut self.config;
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

      config_account.project_count=config_account.project_count.checked_add(1).unwrap();
      
      let fee=config_account.fee;
      if fee > 0 {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
          &self.owner.key(),
              &self.treasury.key(),
              fee,
        );
     anchor_lang::solana_program::program::invoke(
              &ix,
              &[
                  self.owner.to_account_info(),
                  self.treasury.to_account_info(),
                  self.system_program.to_account_info(),
              ],
          )?;
      }
     Ok(())
}
}