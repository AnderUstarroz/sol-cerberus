import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

export async function app_pda(program, appPublicKey) {
  return (
    await PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("app"), appPublicKey.toBuffer()],
      program.programId
    )
  )[0];
}

export async function rule_pda(
  program,
  appPublicKey,
  role,
  resource,
  permission
) {
  return (
    await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(role),
        anchor.utils.bytes.utf8.encode(resource),
        anchor.utils.bytes.utf8.encode(permission),
        appPublicKey.toBuffer(),
      ],
      program.programId
    )
  )[0];
}
