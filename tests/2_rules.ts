import { expect } from "chai";
import { app_pda, READ_PERM, rule_pda } from "./common";
import { APP_ID, PROGRAM, PROVIDER } from "./constants";

describe("2.- Rules", () => {
  let appPDA = null; // Populated on before() block
  const role1 = "Admin";
  const resource1 = "Admin";
  const permission1 = "*";
  let rule1PDA = null; // Populated on before() block

  before(async () => {
    appPDA = await app_pda();
    rule1PDA = await rule_pda(role1, resource1, permission1);
  });

  it("Add rule", async () => {
    let listener = null;
    let [event, _]: any = await new Promise((resolve, _reject) => {
      listener = PROGRAM.addEventListener("RulesChanged", (event, slot) => {
        PROGRAM.removeEventListener(listener);
        resolve([event, slot]);
      });
      PROGRAM.methods
        .addRule({
          namespace: 0,
          role: role1,
          resource: resource1,
          permission: permission1,
          expiresAt: null,
        })
        .accounts({
          app: appPDA,
          rule: rule1PDA,
        })
        .rpc();
    });
    let rule = await PROGRAM.account.rule.fetch(rule1PDA);
    expect(rule.appId.toBase58()).to.equal(APP_ID.toBase58());
    expect(rule.appId.toBase58()).to.equal(event.appId.toBase58());
    expect(rule.role).to.equal(role1);
    expect(rule.resource).to.equal(resource1);
    expect(rule.permission).to.equal(permission1);
    expect(rule.createdAt.toNumber()).to.lessThanOrEqual(
      Math.floor(new Date().getTime() / 1000)
    );

    const rule2PDA = await rule_pda(
      READ_PERM.role,
      READ_PERM.resource,
      READ_PERM.permission
    );
    await PROGRAM.methods
      .addRule({
        namespace: 0,
        role: READ_PERM.role,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
        expiresAt: null,
      })
      .accounts({
        app: appPDA,
        rule: rule2PDA,
      })
      .rpc();
  });

  it("Delete rule", async () => {
    await PROGRAM.methods
      .deleteRule()
      .accounts({
        app: appPDA,
        rule: rule1PDA,
        collector: PROVIDER.wallet.publicKey,
      })
      .rpc();
    try {
      await PROGRAM.account.rule.fetch(rule1PDA);
      throw new Error("The rule should have been deleted at this point!");
    } catch (_err) {
      expect(_err.toString()).to.include(
        "Account does not exist or has no data"
      );
    }
  });
});
