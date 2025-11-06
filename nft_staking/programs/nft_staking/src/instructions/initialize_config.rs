use anchor_lang::prelude::*;

pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,
    //setting up config account
    #[account(
        init,
        payer=admin,
        seeds=[b"stake-config".as_ref()],
        bump,
        space=StakeConfig::DISCRIMINATOR_Len() + StakeConfig::INIT_SPACE,
    )]
    pub config: Account<'info,StakeConfig>,
    //setting up reward mint account
    #[account(
        init,
        payer=admin,
        seeds=[b"reward-vault".as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = config,
    )]
    pub reward_mint: Account<'info,Mint>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>,
}
impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig{
            points_per_stake,
            max_stake,
            freeze_period,
            reward_bump: bumps.reward_mint,
            bumps: bumps.config,
        });
        Ok(())
    }
}