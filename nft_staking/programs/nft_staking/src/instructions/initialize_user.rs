use anchor_lang::prelude::*;
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    //setting up user account
    #[account(
        init,
        payer=user,
        seeds=[b"user-account".as_ref(),user.key().as_ref()],
        bump,
        space=UserAccount::DISCRIMINATOR_Len() + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info,UserAccount>,
    pub system_program: Program<'info,System>,
}
impl<'info> InitializeUser<'info> {
    pub fn initialize_user(
        &mut self,
        bumps: &InitializeUserBumps,
    ) -> Result<()> {
        self.user_account.set_inner(UserAccount{
            amount_staked:0,
            point:0,
            bumps:bumps.user_account,
        });
        Ok(())
    }
}