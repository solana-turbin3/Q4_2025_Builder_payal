use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Project{
    pub owner: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub description: String,
    #[max_len(50)]
   pub ipfs_hash: String,
  
   pub status: Status,
   pub verifier:[Pubkey;10],
    pub project_id: u32,
    pub trust_score: u8,
    pub bump: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status{
    Verified,
    NotVerified,
    Suspended,
}
impl Space for Status{
    const INIT_SPACE:usize=1;
    
}