import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolCerberus } from "../target/types/sol_cerberus";
import { expect } from "chai";
import { app_pda, rule_pda } from "./setup";
import { APP_KEYPAIR } from "./constants";

describe("sol-cerberus", () => {
  const provider = anchor.AnchorProvider.env();
  const unauthorized_keypair = anchor.web3.Keypair.generate();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCerberus as Program<SolCerberus>;

  it("Add rule", async () => {
    const role = "admin";
    const resource = "admin";
    const permission = "*";
    const appPDA = await app_pda(program, APP_KEYPAIR.publicKey);
    const rulePDA = await rule_pda(
      program,
      APP_KEYPAIR.publicKey,
      role,
      resource,
      permission
    );
    await program.methods
      .addRule(role, resource, permission)
      .accounts({
        app: appPDA,
        rule: rulePDA,
      })
      .rpc();
    let rule = await program.account.rule.fetch(rulePDA);
    expect(rule.appId.toBase58()).to.equal(APP_KEYPAIR.publicKey.toBase58());
    expect(rule.role).to.equal(role);
    expect(rule.resource).to.equal(resource);
    expect(rule.permission).to.equal(permission);
  });
});
