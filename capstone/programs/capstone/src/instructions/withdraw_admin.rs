use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::error::CustomError;
use crate::state::Treasury;


#[derive(Accounts)]
pub struct WithDrawAdmin<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
    mut,
    seeds=[b"treasury"],
    bump,
   )]
    pub treasury:Account<'info,Treasury>,
     #[account(
        mut,
        seeds=[b"treasury_vault"],
        bump=treasury.vault_bump,
    )]
   pub vault: SystemAccount<'info>,
    pub system_program:Program<'info,System>,
}
impl<'info>WithDrawAdmin<'info>{
    pub fn withdraw_admin(&mut self,)->Result<()>{
      let amount = self.treasury.admin_paid;
       require!(amount > 0, CustomError::NothingToWithdraw);

// Seeds allow PDA to act as signer
// Required for transferring lamports from a PDA
      let vault_bump = self.treasury.vault_bump;

let seeds = &[
    b"treasury_vault".as_ref(),
    &[vault_bump],
];

let signer_seeds = &[&seeds[..]];

let cpi_ctx = CpiContext::new_with_signer(
    self.system_program.to_account_info(),
    system_program::Transfer {
        from: self.vault.to_account_info(),
        to: self.admin.to_account_info(),
    },
    signer_seeds,
);

      system_program::transfer(cpi_ctx, amount)?;
       self.treasury.total_collected = self.treasury.total_collected.saturating_sub(amount);
      self.treasury.admin_paid=0;
      Ok(())
    }
}