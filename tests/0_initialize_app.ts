import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolCerberus } from "../target/types/sol_cerberus";
import { expect } from "chai";
import { app_pda } from "./setup";

describe("sol-cerberus", () => {
  const provider = anchor.AnchorProvider.env();
  const recovery_keypair = anchor.web3.Keypair.generate();
  const unauthorized_keypair = anchor.web3.Keypair.generate();
  const app_keypair = anchor.web3.Keypair.generate();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCerberus as Program<SolCerberus>;

  it("Initialize App", async () => {
    const [appPDA, _] = await app_pda(program, app_keypair.publicKey);
    try {
      await program.account.app.fetch(appPDA);
    } catch (_err) {
      expect(_err.toString()).to.include("Account does not exist");
    }
    const tx = await program.methods
      .initializeApp({
        id: app_keypair.publicKey,
        recovery: recovery_keypair.publicKey,
      })
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await program.account.app.fetch(appPDA);
    expect(app.id.toBase58()).to.equal(app_keypair.publicKey.toBase58());
    expect(app.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
  });

  it("Update authority", async () => {
    const [appPDA, _] = await app_pda(program, app_keypair.publicKey);
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
        signer: recovery_keypair.publicKey,
      })
      .signers([recovery_keypair])
      .rpc();
    app = await program.account.app.fetch(appPDA);
    expect(app.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
  });
});
