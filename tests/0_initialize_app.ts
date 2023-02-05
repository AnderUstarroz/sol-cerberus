import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolCerberus } from "../target/types/sol_cerberus";
import { expect } from "chai";
import { app_pda } from "./setup";
import { APP_KEYPAIR, RECOVERY_KEYPAIR } from "./constants";

describe("sol-cerberus", () => {
  const provider = anchor.AnchorProvider.env();
  const unauthorized_keypair = anchor.web3.Keypair.generate();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCerberus as Program<SolCerberus>;

  it("Initialize App", async () => {
    const appPDA = await app_pda(program, APP_KEYPAIR.publicKey);
    try {
      await program.account.app.fetch(appPDA);
    } catch (_err) {
      expect(_err.toString()).to.include("Account does not exist");
    }
    const tx = await program.methods
      .initializeApp({
        id: APP_KEYPAIR.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
      })
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await program.account.app.fetch(appPDA);
    expect(app.id.toBase58()).to.equal(APP_KEYPAIR.publicKey.toBase58());
    expect(app.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
  });

  it("Update authority", async () => {
    const appPDA = await app_pda(program, APP_KEYPAIR.publicKey);
    try {
      // Unauthorized users shouldn't be able to update App authority
      await program.methods
        .updateAuthority(unauthorized_keypair.publicKey)
        .accounts({
          app: appPDA,
          signer: unauthorized_keypair.publicKey,
        })
        .signers([unauthorized_keypair])
        .rpc();
      throw new Error(
        "Unauthorized users shouldn't be able to update App authority!"
      );
    } catch (error) {
      expect(error.error.errorCode.code).to.equal(
        "UnauthorizedAuthorityUpdate"
      );
    }
    // Verify current Authority can update the authority of the APP
    await program.methods
      .updateAuthority(unauthorized_keypair.publicKey)
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await program.account.app.fetch(appPDA);
    expect(app.authority.toBase58()).to.equal(
      unauthorized_keypair.publicKey.toBase58()
    );
    // Verify recovery can update the authority of the APP
    await program.methods
      .updateAuthority(provider.wallet.publicKey)
      .accounts({
        app: appPDA,
        signer: RECOVERY_KEYPAIR.publicKey,
      })
      .signers([RECOVERY_KEYPAIR])
      .rpc();
    app = await program.account.app.fetch(appPDA);
    expect(app.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
  });
});
