import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  READ_PERM,
  rule_pda,
} from "./common";
import {
  APP_ID,
  NFTS,
  PROGRAM,
  PROVIDER,
  PROVIDER_WALLET,
  USER_ALLOWED_WALLET,
  USER_WITH_NFTS,
} from "./constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { burnChecked } from "@solana/spl-token";
import { expect } from "chai";

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
        appId: APP_ID,
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
        appId: APP_ID,
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

    // Burn the token to verify use has no access anymore
    await burnChecked(
      PROVIDER.connection, // connection
      USER_WITH_NFTS, // payer
      NFTS.allowedNFT.tokenAddress, // token account
      NFTS.allowedNFT.mintAddress, // mint
      USER_WITH_NFTS.publicKey, // owner
      1, // amount, if your decimals is 9, 10^9 for 1 token
      0
    );
    // Verify user is not allowed anymore
    try {
      await PROGRAM.methods
        .allowed({
          appId: APP_ID,
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
      throw new Error(
        "NFT authenticated users should not be able to access after burning their NFT"
      );
    } catch (e) {
      if (!e.hasOwnProperty("error")) {
        throw e;
      }
      expect(e.error.errorCode.code).to.equal("Unauthorized");
    }
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
        appId: APP_ID,
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
        appId: APP_ID,
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
});
