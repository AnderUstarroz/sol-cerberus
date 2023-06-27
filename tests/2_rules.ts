import { expect } from "chai";
import { app_pda, WRITE_PERM, rule_pda, READ_PERM } from "./common";
import { APP_ID, PROGRAM, PROVIDER, namespaces } from "./constants";

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
    let [event, _]: any = await new Promise((resolve, reject) => {
      listener = PROGRAM.addEventListener("RulesChanged", (event, slot) => {
        PROGRAM.removeEventListener(listener);
        resolve([event, slot]);
      });
      PROGRAM.methods
        .addRule({
          namespace: namespaces.Rule,
          role: role1,
          resource: resource1,
          permission: permission1,
          expiresAt: null,
        })
        .accounts({
          rule: rule1PDA,
          solCerberusApp: appPDA,
          solCerberusRole: null,
          solCerberusRule: null,
          solCerberusRule2: null,
          solCerberusToken: null,
          solCerberusMetadata: null,
          solCerberusSeed: null,
        })
        .rpc();
      // Break infinite loop in case it fails:
      setTimeout(() => {
        reject(new Error("Failed to add rule"));
      }, 2000);
    });
    let rule = await PROGRAM.account.rule.fetch(rule1PDA);
    expect(rule.appId.toBase58()).to.equal(APP_ID.toBase58());
    expect(rule.appId.toBase58()).to.equal(event.appId.toBase58());
    expect(rule.role).to.equal(role1);
    expect(rule.resource).to.equal(resource1);
    expect(rule.permission).to.equal(permission1);

    // Add Write rule
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.Rule,
        role: WRITE_PERM.role,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
        expiresAt: null,
      })
      .accounts({
        rule: await rule_pda(
          WRITE_PERM.role,
          WRITE_PERM.resource,
          WRITE_PERM.permission
        ),
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: null,
      })
      .rpc();

    // Add Read rule
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.Rule,
        role: READ_PERM.role,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
        expiresAt: null,
      })
      .accounts({
        rule: await rule_pda(
          READ_PERM.role,
          READ_PERM.resource,
          READ_PERM.permission
        ),
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: null,
      })
      .rpc();
  });

  it("Delete rule", async () => {
    await PROGRAM.methods
      .deleteRule()
      .accounts({
        rule: rule1PDA,
        collector: PROVIDER.wallet.publicKey,
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: null,
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
