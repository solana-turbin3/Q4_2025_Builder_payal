use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Attestation{
    pub project: Pubkey,
    pub verifier: Pubkey,
    #[max_len(50)]
    pub ipfs_hash: String,
    pub is_valid: bool,
     pub timestamp: i64,
    pub bump: u8,
}