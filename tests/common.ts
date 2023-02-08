import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { APP_KEYPAIR, METADATA_PROGRAM_ID, PROGRAM } from "./constants";

export async function app_pda() {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("app"), APP_KEYPAIR.publicKey.toBuffer()],
      PROGRAM.programId
    )
  )[0];
}

export async function rule_pda(role, resource, permission) {
  return (
    await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(role),
        anchor.utils.bytes.utf8.encode(resource),
        anchor.utils.bytes.utf8.encode(permission),
        APP_KEYPAIR.publicKey.toBuffer(),
      ],
      PROGRAM.programId
    )
  )[0];
}

export async function role_pda(role, address: PublicKey) {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode(role), address.toBuffer()],
      PROGRAM.programId
    )
  )[0];
}

export async function nft_metadata_pda(mint: PublicKey) {
  return (
    await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    )
  )[0];
}

export async function safe_airdrop(
  connection: anchor.web3.Connection,
  destination: anchor.web3.PublicKey,
  lamports = 100_000_000
) {
  // Maximum amount of Lamports per transaction (Devnet allows up to 2SOL per transaction)
  const maxSolPerTx = 2_000_000_000;
  let balance = await connection.getBalance(destination);
  while (balance < lamports) {
    try {
      const latestBlockHash = await connection.getLatestBlockhash();
      // Request Airdrop for user
      await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: await connection.requestAirdrop(
          destination,
          Math.min(lamports - balance, maxSolPerTx)
        ),
      });
      balance = await connection.getBalance(destination);
    } catch {}
  }
}

export const READ_PERM = {
  role: "Authenticated",
  resource: "Homepage",
  permission: "Read",
};
