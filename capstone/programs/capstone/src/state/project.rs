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
    #[max_len(100)]
  pub id: String,
   pub status: Status,
   pub verifier:[Pubkey;10],
    pub project_id: u32,
    pub trust_score: u8,
    pub version: u8,//version is the revision number of the project
    pub bump: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status{
    Verified,
    NotVerified,
    Suspended,
    UnderReview,
    Spam,
}
impl Space for Status{
    const INIT_SPACE:usize=1;
    
}