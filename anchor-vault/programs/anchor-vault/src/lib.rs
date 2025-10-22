use anchor_lang::{
    accounts::signer, prelude::*, system_program::{self, transfer, Transfer}, Bumps
};

declare_id!("4Ex6B11yfRW3svMfbLcrZmTvo3i7jncqLUiHe1u8xEm3");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
                                                
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds= [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer=user,
        seeds=[b"state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
       mut,
       seeds= [b"vault", vault_state.key().as_ref()],
       bump = vault_state.vault_bump,
   )]
    pub vault: SystemAccount<'info>,
    #[account(
    seeds=[b"state", user.key().as_ref()],
    bump = vault_state.state_bump,
   )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds= [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
     mut,
     seeds=[b"state", user.key().as_ref()],
     bump = vault_state.state_bump,
     close=user,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let rent_exempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx,rent_exempt)?;
        self.vault_state.vault_bump=bumps.vault;
         self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
impl<'info> Payment<'info>{
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.system_program.to_account_info();
    let cpi_account=Transfer{
        from:self.user.to_account_info(),
        to:self.vault.to_account_info(),
    };
    let cpi_ctx=CpiContext::new(cpi_program,cpi_account);
    transfer(cpi_ctx,amount)?;
        Ok(())
        }
        pub fn withdraw(&mut self,amount:u64)->Result<()>{
            let cpi_program=self.system_program.to_account_info();
    let cpi_account=Transfer{
        from:self.vault.to_account_info(),
        to:self.user.to_account_info(), 
    };
     let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
    let signer_seeds = &[&seeds[..]];
    let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_account,signer_seeds);
    transfer(cpi_ctx,amount)?;
    Ok(())
    }
}
impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
          let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_context, self.vault.lamports());
         Ok(())
    }
}
#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}