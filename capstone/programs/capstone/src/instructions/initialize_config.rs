use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(Accounts)]
pub struct InitializeConfigAccounts<'info> {
    #[account(
        init,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfigAccounts<'info> {
    pub fn initialize(&mut self, registration_fee: u16, program_id: &Pubkey) -> Result<()> {
        let config_account = &mut self.config;

        let (treasury_pda, _treasury_bump) =
            Pubkey::find_program_address(&[b"treasury".as_ref()], program_id);
        let (_config_pda, config_bump) =
            Pubkey::find_program_address(&[b"config".as_ref()], program_id);

        config_account.admin = self.admin.key();
        config_account.treasury = treasury_pda;
        config_account.verifier_registry = Pubkey::default();
        config_account.fee = registration_fee;
        config_account.project_count = 0;
        config_account.bump = config_bump;

        Ok(())
    }
}