use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub verifier_registry: Pubkey,
    pub fee: u64,
    pub project_count: u32,
    pub bump: u8,
}