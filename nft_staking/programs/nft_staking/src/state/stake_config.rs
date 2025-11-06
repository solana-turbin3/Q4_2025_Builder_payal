use anchor_lang::prelude::*;
#[account]
#[account(InitSpace)]
pub struct StakeConfig{
    pub points_per_stake:u8,
    pub max_stake:u8,
    pub freeze_period:u32,
    pub reward_bump:u8,
    pub bumps:u8,
}