import { BN } from "bn.js";
import { expect } from "chai";
import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  READ_PERM,
  rule_pda,
} from "./common";
import { NFTS, PROGRAM, PROGRAM_TEST_CPI, USER } from "./constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";

describe("4.- Check permissions", () => {
  let appPDA = null; // Populated on before() block
  let rulePDA = null; // Populated on before() block

  before(async () => {
    appPDA = await app_pda();
    rulePDA = await rule_pda(
      READ_PERM.role,
      READ_PERM.resource,
      READ_PERM.permission
    );
  });
  it("Check allowed Authority", async () => {
    const metadataPDA = await nft_metadata_pda(NFTS.allowedNFT.mintAddress);
    const rolePDA = await role_pda(READ_PERM.role, NFTS.allowedNFT.mintAddress);
    const tokenAccountPDA = await getAssociatedTokenAddress(
      NFTS.allowedNFT.mintAddress,
      USER.publicKey
    );
    await PROGRAM.methods
      .allowed({
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusTokenAcc: null,
        solCerberusMetadata: null,
        solCerberusRule: rulePDA,
        solCerberusRole: null,
      })
      .rpc();
  });

  it("Check allowed NFT", async () => {
    const metadataPDA = await nft_metadata_pda(NFTS.allowedNFT.mintAddress);
    const rolePDA = await role_pda(READ_PERM.role, NFTS.allowedNFT.mintAddress);
    const tokenAccountPDA = await getAssociatedTokenAddress(
      NFTS.allowedNFT.mintAddress,
      USER.publicKey
    );
    await PROGRAM.methods
      .allowed({
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusTokenAcc: tokenAccountPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusRule: rulePDA,
        solCerberusRole: rolePDA,
        signer: USER.publicKey,
      })
      .signers([USER])
      .rpc();
  });

  it("Check allowed NFT collection", async () => {
    const metadataPDA = await nft_metadata_pda(
      NFTS.allowedCollection.mintAddress
    );
    const rolePDA = await role_pda(
      READ_PERM.role,
      NFTS.allowedCollection.nft.collection.address
    );
    const tokenAccountPDA = await getAssociatedTokenAddress(
      NFTS.allowedCollection.mintAddress,
      USER.publicKey
    );
    await PROGRAM.methods
      .allowed({
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusTokenAcc: tokenAccountPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusRule: rulePDA,
        solCerberusRole: rolePDA,
        signer: USER.publicKey,
      })
      .signers([USER])
      .rpc();
  });

  it("Check allowed wallet", async () => {
    const rolePDA = await role_pda(READ_PERM.role, USER.publicKey);
    await PROGRAM.methods
      .allowed({
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRule: rulePDA,
        solCerberusRole: rolePDA,
        solCerberusTokenAcc: null,
        solCerberusMetadata: null,
        signer: USER.publicKey,
      })
      .signers([USER])
      .rpc();
  });

  it("Borrame CPI", async () => {
    const rolePDA = await role_pda(READ_PERM.role, USER.publicKey);
    let result = await PROGRAM_TEST_CPI.methods
      .testingCpi({
        solCerverusResource: READ_PERM.resource,
        solCerverusPermission: READ_PERM.permission,
      })
      .accounts({
        signer: USER.publicKey,
        solCerberusApp: appPDA,
        solCerberusRule: rulePDA,
        solCerberusRole: rolePDA,
        solCerberusTokenAcc: null,
        solCerberusMetadata: null,
        solCerberus: PROGRAM.programId,
      })
      .signers([USER])
      .rpc();
    console.log("CPI Result:", result);
  });
});
