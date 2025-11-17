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
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.capstone as Program<Capstone>;
  const wallet = provider.wallet as anchor.Wallet;
  let configPda: PublicKey;
  let treasuryPda: PublicKey;
  let vaultPda: PublicKey;
  let verifierRegistryPda: PublicKey;
  let is_valid: boolean;

  let configBump: number;
  let treasuryBump: number;
  let verifierBump: number;
  let vaultBump: number;

  // project + owner
  let projectOwner: Keypair;
  let projectPda: PublicKey;
  let projectBump: number;
  const projectName = "Eco Water Purifier";
  const projectDesc = "Decentralized water purifier certification system";
  const ipfsHash = "Qm12345abcde";

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
    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_vault")],
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
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Config initialized:", tx);
  });

  it("Registers a new project and transfers fee", async () => {
    // Derive Project PDA using on-chain config.projectCount (robust vs hardcoding)
    const configBefore = await program.account.config.fetch(configPda);
    const projectCount = configBefore.projectCount;
    [projectPda, projectBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("project"),
        projectOwner.publicKey.toBuffer(),
        new anchor.BN(projectCount).toArrayLike(Buffer, "le", 4),
      ],
      program.programId
    );

    const tx = await program.methods
      .registerProject(projectName, projectDesc, ipfsHash)
      .accounts({
        owner: projectOwner.publicKey,
        project: projectPda,
        config: configPda,
        treasury: treasuryPda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([projectOwner])
      .rpc();

    console.log(" Project registered:", tx);
    const projectAccount = await program.account.project.fetch(projectPda);
    const configAccount = await program.account.config.fetch(configPda);
    const treasuryAccount = await provider.connection.getAccountInfo(
      treasuryPda
    );
    assert.equal(
      projectAccount.owner.toBase58(),
      projectOwner.publicKey.toBase58()
    );
    assert.equal(projectAccount.name, projectName);
    assert.equal(projectAccount.description, projectDesc);
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

    const vaultAccount = await provider.connection.getAccountInfo(vaultPda);
    const vaultBalance = vaultAccount.lamports;
    assert.ok(vaultBalance >= registrationFee.toNumber());
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
    console.log("Verifier added:");

    const verifierAccount = await program.account.verifierRegistry.fetch(
      verifierRegistryPda
    );
    const isVerifier = verifierAccount.verifier.find(
      (v: PublicKey) => v.toBase58() === admin.publicKey.toBase58()
    );
    assert.ok(isVerifier, "Verifier should be in the registry");
  });

  it("Verifier verifies the project", async () => {
    // Create and register a verifier for this test
    const verifier = Keypair.generate();
    const txAdd = await program.methods
      .addVerifier(verifier.publicKey)
      .accounts({
        admin: wallet.publicKey,
        verifierRegistry: verifierRegistryPda,
        newVerifier: verifier.publicKey,
      } as any)
      .rpc();
    console.log("Verifier added for verification:", txAdd);

    const airdrop2 = await provider.connection.requestAirdrop(
      verifier.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop2);
    const [attestationPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("attestation"),
        projectPda.toBuffer(),
        verifier.publicKey.toBuffer(),
      ],
      program.programId
    );

    is_valid = true;

    const tx = await program.methods
      .verifyProject(ipfsHash, is_valid)
      .accounts({
        verifier: verifier.publicKey,
        project: projectPda,
        verifierRegistry: verifierRegistryPda,
        attestation: attestationPda,
      } as any)
      .signers([verifier])
      .rpc();
    console.log("Project verified:", tx);

    const projectAccount = await program.account.project.fetch(projectPda);
    console.log("Project status object:", projectAccount.status);

    assert.ok(
      "verified" in projectAccount.status,
      "Project status should be Verified"
    );
    assert.ok(
      projectAccount.trustScore === 10,
      "Trust score should be updated to 10"
    );
    assert.ok(
      !("spam" in projectAccount.status),
      "Project status should not be Spam"
    );
  });
  it("Verifier marks the project as spam", async () => {
    // Create and register a verifier for this test
    const verifier = Keypair.generate();
    const txAdd = await program.methods
      .addVerifier(verifier.publicKey)
      .accounts({
        admin: wallet.publicKey,
        verifierRegistry: verifierRegistryPda,
        newVerifier: verifier.publicKey,
      } as any)
      .rpc();
    console.log("Verifier added for spam marking:", txAdd);

    const airdrop2 = await provider.connection.requestAirdrop(
      verifier.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop2);
    const [attestationPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("attestation"),
        projectPda.toBuffer(),
        verifier.publicKey.toBuffer(),
      ],
      program.programId
    );

    is_valid = false;

    const tx = await program.methods
      .verifyProject(ipfsHash, is_valid)
      .accounts({
        verifier: verifier.publicKey,
        project: projectPda,
        verifierRegistry: verifierRegistryPda,
        attestation: attestationPda,
      } as any)
      .signers([verifier])
      .rpc();
    console.log("Project marked as spam:", tx);

    const projectAccount = await program.account.project.fetch(projectPda);
    console.log(
      "Project status object after spam marking:",
      projectAccount.status
    );

    assert.ok("spam" in projectAccount.status, "Project status should be Spam");
    assert.ok(
      projectAccount.trustScore === 5,
      "Trust score should be decreased to 5"
    );
  });

  it("Prevents duplicate verification by the same verifier", async () => {
    const verifier = Keypair.generate();
    const txAdd = await program.methods
      .addVerifier(verifier.publicKey)
      .accounts({
        admin: wallet.publicKey,
        verifierRegistry: verifierRegistryPda,
        newVerifier: verifier.publicKey,
      } as any)
      .rpc();
      
    console.log("Verifier added for spam marking:", txAdd);

    const airdrop2 = await provider.connection.requestAirdrop(
      verifier.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop2);
    const [attestationPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("attestation"),
        projectPda.toBuffer(),
        verifier.publicKey.toBuffer(),
      ],
      program.programId
    );
    const is_valid = true;
    await program.methods
    .verifyProject(ipfsHash, is_valid)
    .accounts({
      verifier: verifier.publicKey,
      project: projectPda,
      verifierRegistry: verifierRegistryPda,
      attestation: attestationPda,
    } as any)
    .signers([verifier])
    .rpc();

  console.log("✔ First verification succeeded");

  // Second verification [Fails]
  try {
    await program.methods
      .verifyProject(ipfsHash, is_valid)
      .accounts({
        verifier: verifier.publicKey,
        project: projectPda,
        verifierRegistry: verifierRegistryPda,
        attestation: attestationPda,
      } as any)
      .signers([verifier])
      .rpc();

    assert.fail("❌ Duplicate verification was allowed");
  } catch (err: any) {
    console.log("✔ Duplicate verification prevented");
  }
  });

  it("Update project", async () => {
    const tx = await (program as any).methods
      .updateProject(projectName, "this is the updated description", ipfsHash)
      .accounts({
        owner: projectOwner.publicKey,
        project: projectPda,
      } as any)
      .signers([projectOwner])
      .rpc();
    console.log("Project updated:", tx);
    const projectAccount = await program.account.project.fetch(projectPda);
    assert.equal(
      projectAccount.description,
      "this is the updated description",
      "Project description should be updated"
    );
  });
  it("Withdraw admin funds", async () => {
    const treasuryAccount = await program.account.treasury.fetch(treasuryPda);
    const adminPaid = treasuryAccount.adminPaid;
    const initialAdminBalance = await provider.connection.getBalance(
      wallet.publicKey
    );
    const tx = await program.methods
      .withdrawAdmin()
      .accounts({
        admin: wallet.publicKey,
        treasury: treasuryPda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();
    console.log("Admin funds withdrawn:", tx);
    const finalAdminBalance = await provider.connection.getBalance(
      wallet.publicKey
    );
    assert.ok(
      finalAdminBalance > initialAdminBalance,
      "Admin balance should increase after withdrawal"
    );
    assert.ok(
      finalAdminBalance >= initialAdminBalance + adminPaid.toNumber() - 10000,
      "Admin should receive correct amount minus tx fee"
    );
  });
  it("Withdraw verifier funds", async () => {
    const verifier = Keypair.generate();
    await program.methods
      .addVerifier(verifier.publicKey)
      .accounts({
        admin: wallet.publicKey,
        verifierRegistry: verifierRegistryPda,
        newVerifier: verifier.publicKey,
      } as any)
      .rpc();

    const drop = await provider.connection.requestAirdrop(
      verifier.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(drop);

    const [attestationPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("attestation"),
        projectPda.toBuffer(),
        verifier.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .verifyProject(ipfsHash, true) 
      .accounts({
        verifier: verifier.publicKey,
        project: projectPda,
        verifierRegistry: verifierRegistryPda,
        attestation: attestationPda,
      } as any)
      .signers([verifier])
      .rpc();
    const treasuryBefore = await program.account.treasury.fetch(treasuryPda);
    const verifierPool = treasuryBefore.verifierPool;

    assert.ok(
      verifierPool.gt(new anchor.BN(0)),
      "Verifier pool should not be zero"
    );

    const initialBalance = await provider.connection.getBalance(
      verifier.publicKey
    );

    const tx = await program.methods
      .withdrawVerifier()
      .accounts({
        verifier: verifier.publicKey,
        treasury: treasuryPda,
        verifierRegistry: verifierRegistryPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([verifier])
      .rpc();

    console.log("Verifier funds withdrawn:");

    const finalBalance = await provider.connection.getBalance(
      verifier.publicKey
    );

    assert.ok(
      finalBalance > initialBalance,
      "Verifier should receive lamports"
    );
  });
});
