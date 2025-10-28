use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::Config;
use crate::errors::AmmError;
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,
    pub mint_x:Account<'info, Mint>,
    pub mint_y:Account<'info, Mint>,
    #[account(
        init,
        payer=admin,
        seeds=[b"config"],
        bump,
        space = Config::INIT_SPACE,
    )]  
    pub config:Account<'info, Config>,
    #[account(
        init,
        payer=admin,
        mint::decimals=6,
        mint::authority=config.key(),
        seeds=[b"lp",config.key().as_ref()],
        bump,
    )]
    pub mint_lp:Account<'info, Mint>,
    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_x,
        associated_token::authority=config,
        associated_token::token_program=token_program,
    )]
    pub vault_x:Account<'info, TokenAccount>,
    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_y,
        associated_token::authority=config,
        associated_token::token_program=token_program,
    )]
    pub vault_y:Account<'info, TokenAccount>,

    pub token_program:program<'info, Token>,
    pub associated_token_program:program<'info,AssociatedToken>,
    pub system_program:program<'info,System>,

}
impl<'info>Initialize<'info>{
    pub fn initialize(
        &mut self,
        fee:u16,
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    )->Result<()>{
        self.config.set_inner(Config{
            authority,
            mint_x:self.mint_x.key(),
            mint_y:self.mint_y.key(),
            fee,
            config_bump:bumps.config,
            lp_bumps:bumps.mint_lp,

        });
        Ok(())

    }
}
//This creates a new instance of your Config struct, fills it with values taken from your accounts,and saves it into the blockchain.