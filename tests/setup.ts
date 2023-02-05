import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

export async function app_pda(program, appPublicKey) {
  return await PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("app"), appPublicKey.toBuffer()],
    program.programId
  );
}
