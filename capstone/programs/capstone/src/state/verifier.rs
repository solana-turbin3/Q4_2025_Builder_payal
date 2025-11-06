use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct VerifierRegistry{
   pub admin: Pubkey,
   #[max_len(10)]
   pub verifier:Vec<Pubkey>,
   pub bump: u8,
}
