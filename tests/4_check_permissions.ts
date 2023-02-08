import { BN } from "bn.js";
import { expect } from "chai";
import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  READ_PERM,
  rule_pda,
} from "./common";
import { NFTS, PROGRAM, USER } from "./constants";
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
        tokenAccount: tokenAccountPDA,
        metadata: metadataPDA,
        rule: rulePDA,
        role: rolePDA,
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
        tokenAccount: tokenAccountPDA,
        metadata: metadataPDA,
        rule: rulePDA,
        role: rolePDA,
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
        tokenAccount: null,
        metadata: null,
        rule: rulePDA,
        role: rolePDA,
        signer: USER.publicKey,
      })
      .signers([USER])
      .rpc();
  });
});
