use anchor_lang::prelude::*;

declare_id!("CccyepTSBncAWZ2uch1sRJJkXbaWvSQsndz4E1Md7g86");

#[program]
pub mod mplxcore {
    use super::*;

    pub fn whitelist_creator(ctx:Context<WhitelistCreator>) -> Result<()> {
        ctx.accounts.whitelist_creator()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
