use anchor_lang::prelude::*;
use crate::error::MplxcoreError;

#[accounts]
#[derive(InitSpace)]
pub struct WhitelistedCreatorsInit<'info> {
    pub creators:[Pubkey; 10],
    #[max_len=10]
    pub creators1:Vec<Pubkey>,

pub num_creators: u8,
pub bump: u8,
}
impl WhitelistCreators{
    pub fn contains(&self,creator:&AccountInfo)->bool{
        self.creators[..self.num_creators as usize].contains(creator.key)
    }
    pub fn whitelist_creator(&mut self, creator:&AccountInfo)->Result<()>{
        if self.num_creators as usize >= self.creators.len(){
            return err!(MplxcoreError::CreatorListFull);
        }
       if self.contains(creator){
            return err!(MplxcoreError::CreatorAlreadyWhitelisted);
        }
        self.creators[self.num_creators as usize]=creator.key();
        self.num_creators+=1;
        Ok(())
    }
}
