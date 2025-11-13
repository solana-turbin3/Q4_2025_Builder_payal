// use anchor_lang::prelude::*;
// use anchor_lang::system_program; 
// use crate::state::Treasury;
// #[derive(Accounts)]
// pub struct WithDrawVerifier<'info>{
//     #[account(mut)]
//     pub verifier:Signer<'info>,

//     #[account(
//     mut,
//     seeds=[b"treasury"],
//     bump,
//    )]
//     pub treasury:Account<'info,crate::state::Treasury>,
//     pub system_program:Program<'info,System>,
// }
// impl<'info>WithDrawVerifier<'info>{
//     pub fn withdraw_verifier(&mut self)->Result<()>{
//         let amount = self.treasury.verifier_pool/self.treasury;
//         let seeds=&[b"treasury".as_ref(),&[self.treasury.bump]];
//         let signer_seeds=&[&seeds[..]];
//         let cpi_ctx = CpiContext::new_with_signer(
//             self.system_program.to_account_info(),
//             system_program::Transfer {
//                 from: self.treasury.to_account_info(),
//                 to: self.verifier.to_account_info(),
//             },
//             signer_seeds,
//         );
//         system_program::transfer(cpi_ctx, amount)?;
//         self.treasury.verifier_paid=0;
//         Ok(())
//     }
// }