use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;

use instructions::*;
use state::*;

declare_id!("S4aa8LabXaNAcqYYM9jtotQhQAyH6Ndackz4Bd5BqjU");

#[program]
pub mod capstone {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfigAccounts>, registration_fee: u64) -> Result<()> {
        ctx.accounts.initialize(registration_fee, ctx.program_id)
    }
    pub fn register_project(ctx: Context<RegisterProjectAccounts>,name:String,description:String,ipfs_hash:String)->Result<()>{
        ctx.accounts.register_project(name,description,ipfs_hash)
    }
}
