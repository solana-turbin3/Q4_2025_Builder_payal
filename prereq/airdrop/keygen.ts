import { createKeyPairSignerFromBytes } from "@solana/kit";
import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
const keypair = await crypto.subtle.generateKey({ name: "Ed25519" }, true, [
  "sign",
  "verify",
]);
const privateKeyJwk = await crypto.subtle.exportKey("jwk", keypair.privateKey);
const privateKeyBase64 = privateKeyJwk.d;
if (!privateKeyBase64) throw new Error("Failed to get private keybytes");
const privateKeyBytes = new Uint8Array(Buffer.from(privateKeyBase64, "base64"));
const publicKeyBytes = new Uint8Array(
  await crypto.subtle.exportKey("raw", keypair.publicKey)
);
const keypairBytes = new Uint8Array([...privateKeyBytes, ...publicKeyBytes]);
const signer = await createKeyPairSignerFromBytes(keypairBytes);
console.log(`You have generated a new Solana wallet:
${signer.address}`);
console.log(`To save your wallet, copy and paste the following into a
JSON file: [${keypairBytes}]`)
function wallettobase58(kp:Keypair){
    let wallet=kp.secretKey;
    let base58=bs58.encode(wallet);
    return base58;
}
function base58towallet(kp: Keypair){
    let base58 = wallettobase58(kp);
    let wallet = bs58.decode(base58);

    return wallet;
}
const a = wallettobase58(Keypair.fromSecretKey(keypairBytes));
const b = base58towallet(Keypair.fromSecretKey(keypairBytes));
console.log("wallettobase58", a, "base58towallet  ", b);