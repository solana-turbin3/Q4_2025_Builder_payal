**Turbin3 Rust Prerequisites**

Program ID (Devnet): TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM
Status: All tests successfully implemented and deployed on Solana Devnet

**Overview**

This repository demonstrates a complete workflow for interacting with Solana wallets and programs using Rust:

 -Generate and manage Solana keypairs.

 -Request SOL airdrops and transfer tokens on Devnet.

 -Interact directly with on-chain programs.

 -Mint an NFT via the Turbin3 enrollment program.

**Requirements**

 -Wallet Generation

 -Create new Solana keypairs in Rust.

 -Store private keys in JSON format.

 -Convert between JSON and Phantom’s Base58 format for cross-wallet compatibility.

 -Claim Devnet Tokens

 -Connect to Solana Devnet RPC.

 -Request SOL airdrops into your generated wallet.

 -Token Transfers

 -Transfer SOL to a registered Turbin3 wallet.

 -Verify signatures before sending transactions.

 -Empty wallet balances safely, accounting for transaction fees.


States and Flow
1. Initialize Wallet

Generate a keypair.

Save it to dev-wallet.json.

2. Fund Wallet

Request a 2 SOL airdrop on Devnet.

3. Transfer SOL

Send 0.1 SOL to your Turbin3 wallet.

Empty the remaining balance into the Turbin3 wallet, leaving only fees.

4. Submit Completion

Call the submit_rs instruction with:

PDA

Collection

System program

Metaplex Core

Mint your NFT as proof of completion.

