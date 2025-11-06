use anchor_lang::prelude::*;
#[account]
#[account(InitSpace)]
pub struct StakeAccount{
   
    pub staked_at:u64,
    pub mint:Pubkey,
    pub owner:Pubkey,
     pub bump:u8,
}