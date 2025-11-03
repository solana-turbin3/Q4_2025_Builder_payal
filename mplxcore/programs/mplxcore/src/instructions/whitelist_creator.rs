use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WhitelistCreator {
    #[account(mut)]
    pub payer:Signer<'info>,
    pub creator:SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space=WhitelistCreators::DISCRIMINATOR.len()+WhitelistedCreators::INIT_SPACE,
        seeds = [b"whitelist"],
        bump,
    )]
    pub whitelisted_creators:Account<'info, WhitelistedCreators>,
    pub system_program:Program<'info, System>,
    #[account(constraint =this_program.programdata_address()?==Some(program_data.key()))]
    pub this_program:Program<'info, mplxcore>,
    #[account(constraint=program_data.upgrade_authority_address==Some(payer.key()))]
    pub program_data:Account<'info, ProgramData>,
}
impl<'info> WhitelistCreator<'info> {
    pub fn whitelist_creator(&mut self) -> Result<()> {
        Ok(())
    }
}