import { app_pda, role_pda, WRITE_PERM, rule_pda, READ_PERM } from "./common";
import {
  ANOTHER_WALLET,
  APP_ID,
  PROGRAM,
  ALLOWED_WALLET,
  addressType,
  namespaces,
} from "./constants";
import { expect } from "chai";

describe("5.- Allow assign roles", () => {
  let appPDA = null; // Populated on before() block
  let writeRulePDA = null; // Populated on before() block
  let allowedWalletRole = null; // Populated on before() block
  let newWalletRole = null; // Populated on before() block

  before(async () => {
    appPDA = await app_pda();
    writeRulePDA = await rule_pda(
      WRITE_PERM.role,
      WRITE_PERM.resource,
      WRITE_PERM.permission
    );
    allowedWalletRole = await role_pda(
      WRITE_PERM.role,
      ALLOWED_WALLET.publicKey
    );
    newWalletRole = await role_pda(WRITE_PERM.role, ANOTHER_WALLET.publicKey);
  });

  it("Wallet not allowed to assign role", async () => {
    try {
      // ALLOWED_WALLET does not have permission to assign  the "Authenticated" role
      await PROGRAM.methods
        .assignRole({
          address: ANOTHER_WALLET.publicKey,
          role: WRITE_PERM.role,
          addressType: addressType.Wallet,
          expiresAt: null,
        })
        .accounts({
          role: newWalletRole,
          solCerberusApp: appPDA,
          solCerberusRole: allowedWalletRole,
          solCerberusRule: null,
          solCerberusToken: null,
          solCerberusMetadata: null,
          authority: ALLOWED_WALLET.publicKey,
        })
        .signers([ALLOWED_WALLET])
        .rpc();
      throw Error("Unauthorized wallets shouldn't be allowed to assign roles!");
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
  });

  it("Wallet allowed to assign role", async () => {
    let rulePDA = await rule_pda(
      WRITE_PERM.role,
      "Wallet",
      WRITE_PERM.role,
      namespaces.AssignRole
    );
    // Allow role "Authenticated" to assign the same role "Authenticated" to other wallets
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AssignRole,
        role: WRITE_PERM.role,
        resource: "Wallet",
        permission: WRITE_PERM.role,
        expiresAt: null,
      })
      .accounts({
        rule: rulePDA,
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
      })
      .rpc();

    // Assign role "Authenticated" to another wallet
    await PROGRAM.methods
      .assignRole({
        role: WRITE_PERM.role,
        address: ANOTHER_WALLET.publicKey,
        addressType: addressType.Wallet,
        expiresAt: null,
      })
      .accounts({
        role: newWalletRole,
        solCerberusApp: appPDA,
        solCerberusRole: allowedWalletRole,
        solCerberusRule: rulePDA,
        solCerberusToken: null,
        solCerberusMetadata: null,
        authority: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();

    // Verify the new wallet is finally allowed to write
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: newWalletRole,
        solCerberusRule: writeRulePDA,
        solCerberusToken: null,
        solCerberusMetadata: null,
        signer: ANOTHER_WALLET.publicKey,
      })
      .signers([ANOTHER_WALLET])
      .rpc();
  });
});
