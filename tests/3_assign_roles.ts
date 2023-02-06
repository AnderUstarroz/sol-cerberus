import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolCerberus } from "../target/types/sol_cerberus";
import { expect } from "chai";
import { app_pda, rule_pda } from "./setup";
import { APP_KEYPAIR } from "./constants";

describe("3.- Assign roles", () => {
  const provider = anchor.AnchorProvider.env();
  const unauthorized_keypair = anchor.web3.Keypair.generate();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCerberus as Program<SolCerberus>;

  it("Delete rule", async () => {
    // const role = "admin";
    // const resource = "admin";
    // const permission = "*";
    // const appPDA = await app_pda(program, APP_KEYPAIR.publicKey);
    // const rulePDA = await rule_pda(
    //   program,
    //   APP_KEYPAIR.publicKey,
    //   role,
    //   resource,
    //   permission
    // );
    // await program.methods
    //   .deleteRule(role, resource, permission)
    //   .accounts({
    //     app: appPDA,
    //     rule: rulePDA,
    //     destination: provider.wallet.publicKey,
    //   })
    //   .rpc();
    // try {
    //   await program.account.rule.fetch(rulePDA);
    //   throw new Error("The rule should have been deleted at this point!");
    // } catch (_err) {
    //   expect(_err.toString()).to.include(
    //     "Account does not exist or has no data"
    //   );
    // }
  });
});
