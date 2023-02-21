import { BN } from "bn.js";
import { expect } from "chai";
import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  READ_PERM,
  rule_pda,
} from "./common";
import {
  NFTS,
  PROGRAM,
  PROGRAM_TEST_CPI,
  USER_ALLOWED_WALLET,
  USER_WITH_NFTS,
} from "./constants";
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
      USER_WITH_NFTS.publicKey
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
        signer: USER_WITH_NFTS.publicKey,
      })
      .signers([USER_WITH_NFTS])
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
      USER_WITH_NFTS.publicKey
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
        signer: USER_WITH_NFTS.publicKey,
      })
      .signers([USER_WITH_NFTS])
      .rpc();
  });

  it("Check allowed wallet", async () => {
    const rolePDA = await role_pda(
      READ_PERM.role,
      USER_ALLOWED_WALLET.publicKey
    );
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
        signer: USER_ALLOWED_WALLET.publicKey,
      })
      .signers([USER_ALLOWED_WALLET])
      .rpc();
  });

  it("Borrame CPI", async () => {
    const metadataPDA = await nft_metadata_pda(NFTS.allowedNFT.mintAddress);
    const rolePDA = await role_pda(READ_PERM.role, NFTS.allowedNFT.mintAddress);
    const tokenAccountPDA = await getAssociatedTokenAddress(
      NFTS.allowedNFT.mintAddress,
      USER_WITH_NFTS.publicKey
    );

    await PROGRAM_TEST_CPI.methods
      .testingCpi()
      .accounts({
        signer: USER_WITH_NFTS.publicKey,
        solCerberusApp: appPDA,
        solCerberusRule: rulePDA,
        solCerberusRole: rolePDA,
        solCerberusTokenAcc: tokenAccountPDA,
        solCerberusMetadata: metadataPDA,
        solCerberus: PROGRAM.programId,
      })
      .signers([USER_WITH_NFTS])
      .rpc();
  });
});
