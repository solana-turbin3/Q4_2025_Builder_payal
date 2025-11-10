import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import { expect } from "chai";

describe("capstone", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.capstone as Program<Capstone>;
let configPda: anchor.web3.PublicKey;
let verifierRegistryPda:  anchor.web3.PublicKey;
let treasuryPda: anchor.web3.PublicKey;

  it("Initializes the Config and VerifierRegistry PDAs", async () => {
    // Add your test here.
    configPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];
    verifierRegistryPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("verifier")],
      program.programId
    )[0];
    treasuryPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    )[0];
    const tx = await program.methods.initializeConfig(new anchor.BN(10000000)).accounts({
      config:configPda,
      verifierRegistry:verifierRegistryPda,
      admin:program.provider.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId,
    } as any).rpc();
    console.log("Your transaction signature", tx);
    const configAccount = await program.account.config.fetch(configPda);
    console.log("Config Account:", configAccount);
    expect(configAccount.admin.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    expect(configAccount.fee.toNumber()).to.equal(10000000);
     expect(configAccount.projectCount).to.equal(0);
     console.log("Config initialized successfully");
    const verifierRegistryAccount = await program.account.verifierRegistry.fetch(verifierRegistryPda);
    expect(verifierRegistryAccount.verifier.length).to.equal(0);
    console.log("Verifier Registry Account:", verifierRegistryAccount);
  });

  it("Adds a verifier to the Verifier Registry",async()=>{
    
  })
});

