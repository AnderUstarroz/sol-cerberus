import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  WRITE_PERM,
  rule_pda,
  READ_PERM,
} from "./common";
import {
  ANOTHER_WALLET,
  APP_ID,
  NFTS,
  PROGRAM,
  PROVIDER,
  ALLOWED_WALLET,
  WALLET_WITH_NFTS,
  addressType,
  namespaces,
} from "./constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { burnChecked } from "@solana/spl-token";
import { expect } from "chai";
import * as anchor from "@project-serum/anchor";
import { PublicKey } from "@metaplex-foundation/js";

describe("6.- Allow add rules", () => {
  let appPDA: PublicKey | null = null; // Populated on before() block
  let allowedWalletRolePDA = null; // Populated on before() block
  let newRulePDA: PublicKey | null = null; // Populated on before() block
  let nsRoleRulePDA: PublicKey | null = null; // Populated on before() block
  let resourcePermRulePDA: PublicKey | null = null; // Populated on before() block
  const newResource = "MyNewResource";
  const newPerm = "Edit";

  before(async () => {
    appPDA = await app_pda();
    allowedWalletRolePDA = await role_pda(
      WRITE_PERM.role,
      ALLOWED_WALLET.publicKey
    );
    newRulePDA = await rule_pda(WRITE_PERM.role, newResource, newPerm);
    nsRoleRulePDA = await rule_pda(
      WRITE_PERM.role,
      `${namespaces.Rule}`,
      WRITE_PERM.role,
      namespaces.AddRuleNSRole
    );
    resourcePermRulePDA = await rule_pda(
      WRITE_PERM.role,
      newResource,
      newPerm,
      namespaces.AddRuleResourcePerm
    );
  });

  it("Wallet not allowed to add rule", async () => {
    try {
      await PROGRAM.methods
        .addRule({
          namespace: namespaces.Rule,
          role: WRITE_PERM.role,
          resource: newResource,
          permission: newPerm,
          expiresAt: null,
        })
        .accounts({
          rule: newRulePDA,
          solCerberusApp: appPDA,
          solCerberusRole: allowedWalletRolePDA,
          solCerberusRule: null,
          solCerberusRule2: null,
          solCerberusToken: null,
          solCerberusMetadata: null,
          authority: ALLOWED_WALLET.publicKey,
        })
        .signers([ALLOWED_WALLET])
        .rpc();
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
  });

  it("Add rule to allow creation of Namespace and Role", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Namespace: Rule (The kind of namespace of the permission)
    // - Roles of type: "Authenticated" (The role to which the permission could be applied)
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AddRuleNSRole,
        role: WRITE_PERM.role,
        resource: `${namespaces.Rule}`,
        permission: WRITE_PERM.role,
        expiresAt: null,
      })
      .accounts({
        rule: nsRoleRulePDA,
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
      })
      .rpc();
  });

  it("Add rule to allow creation of Resource and Permission", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Resource: Rule (The kind of namespace of the permission)
    // - Permission: "Authenticated" (The role to which the permission could be applied)
    await PROGRAM.methods
      .addRule({
        namespace: namespaces.AddRuleResourcePerm,
        role: WRITE_PERM.role,
        resource: newResource,
        permission: newPerm,
        expiresAt: null,
      })
      .accounts({
        rule: resourcePermRulePDA,
        solCerberusApp: appPDA,
        solCerberusRole: null,
        solCerberusRule: null,
        solCerberusRule2: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
      })
      .rpc();
  });

  it("Wallet allowed to add rule", async () => {
    // Allows the role "Authenticated" to create following permission:
    // - Role:  "Authenticated" (The role receiving the permission)
    // - Namespace: Rule (The kind of namespace of the permission)
    // - Roles of type: "Authenticated" (The role to which the permission could be applied)

    await PROGRAM.methods
      .addRule({
        namespace: namespaces.Rule,
        role: WRITE_PERM.role,
        resource: newResource,
        permission: newPerm,
        expiresAt: null,
      })
      .accounts({
        rule: await rule_pda(
          WRITE_PERM.role,
          newResource,
          newPerm,
          namespaces.Rule
        ),
        solCerberusApp: appPDA,
        solCerberusRole: allowedWalletRolePDA,
        solCerberusRule: nsRoleRulePDA,
        solCerberusRule2: resourcePermRulePDA,
        solCerberusToken: null,
        solCerberusMetadata: null,
        authority: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();
  });
});
