use crate::instructions::verify_project;
use crate::state::Config;
use crate::state::Treasury;
use crate::state::VerifierRegistry;
use anchor_lang::prelude::*;
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
    #[account(
        init,
        payer=admin,
        space=VerifierRegistry::DISCRIMINATOR.len() + VerifierRegistry::INIT_SPACE,
        seeds = [b"verifier"],
        bump,
    )]
    pub verifier: Account<'info, VerifierRegistry>,
    #[account(
    init,
    payer = admin,
    space = Treasury::DISCRIMINATOR.len() + Treasury::INIT_SPACE,
    seeds = [b"treasury"],
    bump,
)]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfigAccounts<'info> {
    pub fn initialize(&mut self, registration_fee: u64, program_id: &Pubkey) -> Result<()> {
        let config_account = &mut self.config;
        let verifier_registry = &mut self.verifier;
        let treasury_account = &mut self.treasury;

        let (treasury_pda, treasury_bump) =
            Pubkey::find_program_address(&[b"treasury".as_ref()], program_id);
        let (_config_pda, config_bump) =
            Pubkey::find_program_address(&[b"config".as_ref()], program_id);
        let (_verifier_pda, verifier_bump) =
            Pubkey::find_program_address(&[b"verifier".as_ref()], program_id);

        verifier_registry.admin = self.admin.key();
        verifier_registry.verifier = vec![];
        verifier_registry.bump = verifier_bump;

        treasury_account.total_collected = 0;
        treasury_account.admin_paid = 0;
        treasury_account.verifier_pool = 0;
        treasury_account.bump = treasury_bump;

        config_account.admin = self.admin.key();
        config_account.treasury = treasury_pda;
        config_account.verifier_registry = verifier_registry.key();
        config_account.fee = registration_fee;
        config_account.project_count = 0;
        config_account.bump = config_bump;

        Ok(())
    }
}
