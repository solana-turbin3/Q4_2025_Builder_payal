use anchor_lang::prelude::*;
#[account]
#[account(InitSpace)]
pub struct UserAccount{
    pub amount_staked:u8,
    pub point:u32,
    bumps:u8,

}