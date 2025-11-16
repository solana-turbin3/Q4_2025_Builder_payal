use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub total_collected: u64,
    pub verifier_pool: u64,
    pub admin_paid: u64,
    pub bump: u8,
     pub vault_bump: u8,
}
