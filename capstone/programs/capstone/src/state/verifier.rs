use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct VerifierRegistry{
   pub admin: Pubkey,
   #[max_len(10)]
   pub verifier:Vec<Pubkey>,
   #[max_len(10)]
   pub attestation_counts:Vec<u64>,
   pub total_attestations:u64,
   pub bump: u8,
}
