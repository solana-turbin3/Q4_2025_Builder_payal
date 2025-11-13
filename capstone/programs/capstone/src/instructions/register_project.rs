use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::config;
use crate::state::Project;
use crate::state::Status;
use crate::state::Config;
use crate::state::treasury;
use crate::state::verifier;
use crate::state::Treasury;
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
    pub treasury:Account<'info,Treasury>,
  #[account(
    init,
    payer=owner,
    seeds=[b"project",owner.key().as_ref(),
            &config.project_count.to_le_bytes()],
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
        let treasury=&mut self.treasury;
      let (_pda, project_bump) = Pubkey::find_program_address(
          &[b"project", self.owner.key.as_ref(), &config_account.project_count.to_le_bytes(),],
          &crate::ID,
      );

        project_account.owner=self.owner.key();
       project_account.project_id = config_account.project_count;
        project_account.name=name;
        project_account.description=description;
        project_account.ipfs_hash=ipfs_hash;
        project_account.status=Status::NotVerified;
        project_account.verifier=[Pubkey::default();10];
       
        project_account.trust_score=0;
        project_account.bump=project_bump;

      config_account.project_count=config_account.project_count.checked_add(1).unwrap();
      
       let fee = config_account.fee;
        if fee > 0 {
            let cpi_ctx = CpiContext::new(
                self.system_program.to_account_info(),
                system_program::Transfer {
                    from: self.owner.to_account_info(),
                    to: treasury.to_account_info(),
                },
            );
            system_program::transfer(cpi_ctx, fee)?;
        }

      let admin_share=fee*40/100;
      let verifier_share=fee*60/100;
    treasury.total_collected = treasury.total_collected.saturating_add(fee);
        treasury.admin_paid = treasury.admin_paid.saturating_add(admin_share);
        treasury.verifier_pool = treasury.verifier_pool.saturating_add(verifier_share);

     Ok(())
}
}