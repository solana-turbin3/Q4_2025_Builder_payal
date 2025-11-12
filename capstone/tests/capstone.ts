import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { assert } from "chai";

describe("capstone", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.capstone as Program<Capstone>;
  const wallet = provider.wallet as anchor.Wallet;
  let configPda: PublicKey;
  let treasuryPda: PublicKey;
  let verifierRegistryPda: PublicKey;
  let ipfs_hash: string;
  let is_valid: boolean;

  let configBump: number;
  let treasuryBump: number;
  let verifierBump: number;

  // project + owner
  let projectOwner: Keypair;
  let projectPda: PublicKey;
  let projectBump: number;
  const registrationFee = new anchor.BN(0.5 * LAMPORTS_PER_SOL);
  before(async () => {
    projectOwner = Keypair.generate();
    const airdrop1 = await provider.connection.requestAirdrop(
      projectOwner.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop1);
    [configPda, configBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
    [treasuryPda, treasuryBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );
    [verifierRegistryPda, verifierBump] =
      await PublicKey.findProgramAddressSync(
        [Buffer.from("verifier")],
        program.programId
      );
    const tx = await program.methods
      .initializeConfig(registrationFee)
      .accounts({
        admin: wallet.publicKey,
        config: configPda,
        verifier: verifierRegistryPda,
        treasury: treasuryPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Config initialized:", tx);
  });

  it("Registers a new project and transfers fee", async () => {
    // Derive Project PDA
    const projectCount = 0; // first project
    [projectPda, projectBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("project"),
        projectOwner.publicKey.toBuffer(),
        new anchor.BN(projectCount).toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const projectName = "Eco Water Purifier";
    const projectDesc = "Decentralized water purifier certification system";
    const ipfsHash = "Qm12345abcde";

    const tx = await program.methods
      .registerProject(projectName, projectDesc, ipfsHash)
      .accounts({
        owner: projectOwner.publicKey,
        project: projectPda,
        config: configPda,
        treasury: treasuryPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([projectOwner])
      .rpc();

    console.log(" Project registered:", tx);

    // Fetch project data
    const projectAccount = await program.account.project.fetch(projectPda);
    const configAccount = await program.account.config.fetch(configPda);
    const treasuryAccount = await provider.connection.getAccountInfo(
      treasuryPda
    );

    // Assertions
    assert.equal(
      projectAccount.owner.toBase58(),
      projectOwner.publicKey.toBase58()
    );
    assert.equal(projectAccount.name, projectName);
    assert.equal(projectAccount.description, projectDesc);
    // Cast status to any to safely access the boolean flag returned by the account
    assert.ok(
      "notVerified" in projectAccount.status,
      "Project status should be NotVerified"
    );

    assert.ok(
      projectAccount.trustScore === 0,
      "Initial trust score should be 0"
    );
    assert.equal(
      configAccount.projectCount,
      1,
      "Project count should increment"
    );

    // Check Treasury received fee
    const treasuryBalance = treasuryAccount.lamports;
    assert.ok(
      treasuryBalance >= registrationFee.toNumber(),
      "Treasury PDA should receive registration fee"
    );
  });
  it("Adds a verifier to the registry", async () => {
    const admin = Keypair.generate();
    [verifierRegistryPda, verifierBump] =
      await PublicKey.findProgramAddressSync(
        [Buffer.from("verifier")],
        program.programId
      );
    const tx = await program.methods
      .addVerifier(admin.publicKey)
      .accounts({
        admin: wallet.publicKey,
        verifierRegistry: verifierRegistryPda,
        newVerifier: admin.publicKey,
      } as any)
      .rpc();
    console.log("Verifier added:", tx);

    const verifierAccount = await program.account.verifierRegistry.fetch(
      verifierRegistryPda
    );
    const isVerifier = verifierAccount.verifier.find(
      (v: PublicKey) => v.toBase58() === admin.publicKey.toBase58()
    );
    assert.ok(isVerifier, "Verifier should be in the registry");
  });
  
   });

