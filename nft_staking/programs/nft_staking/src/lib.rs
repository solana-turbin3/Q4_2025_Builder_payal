use anchor_lang::prelude::*;

declare_id!("EBuuDsMkXx1fK2iQ889GB8mfynBdQStY1eR86w133Y8B");

#[program]
pub mod nft_staking {
   
    use super::*;

    pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    points_per_stake:u8,
    max_stake:u8,
    freeze_period:u32,
    ) -> Result<()> {
       ctx.accounts.initialize_config(ctx, points_per_stake, max_stake, freeze_period,&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
