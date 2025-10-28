use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer_checked, Mint, MintTo, Token, TokenAccount, TransferChecked},
};
#[derive(Accounts)]     
pub struct Deposit<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(mint::token_program=token_program)]
    pub mint_x:Account<'info, Mint>,
    #[account(mint::token_program=token_program)]
    pub mint_y:Account<'info, Mint>,
    
  #[account(
   seeds=[b"config"],
   bump=config
  )]
    
}