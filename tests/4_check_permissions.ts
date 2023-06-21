import {
  app_pda,
  role_pda,
  nft_metadata_pda,
  WRITE_PERM,
  rule_pda,
  READ_PERM,
  seed_pda,
} from "./common";
import {
  APP_ID,
  NFTS,
  PROGRAM,
  PROVIDER,
  ALLOWED_WALLET,
  WALLET_WITH_NFTS,
  namespaces,
  FEE,
} from "./constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { burnChecked } from "@solana/spl-token";
import { expect } from "chai";
import { BN } from "bn.js";
import { Transaction } from "@solana/web3.js";

describe("4.- Check permissions", () => {
  let appPDA = null; // Populated on before() block
  let writeRulePDA = null; // Populated on before() block
  let readRulePDA = null; // Populated on before() block
  let walletSeedPDA = null; // Populated on before() block
  let WalletWithNFTSeedPDA = null; // Populated on before() block

  before(async () => {
    appPDA = await app_pda();
    writeRulePDA = await rule_pda(
      WRITE_PERM.role,
      WRITE_PERM.resource,
      WRITE_PERM.permission
    );
    readRulePDA = await rule_pda(
      READ_PERM.role,
      READ_PERM.resource,
      READ_PERM.permission
    );
    walletSeedPDA = await seed_pda(ALLOWED_WALLET.publicKey);
    WalletWithNFTSeedPDA = await seed_pda(WALLET_WITH_NFTS.publicKey);
  });

  it("Check allowed Authority", async () => {
    const before_balance = await PROVIDER.connection.getBalance(
      PROVIDER.publicKey
    );
    // Allowed to Write
    const ix = PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRule: writeRulePDA,
        solCerberusRole: null,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: null,
      });
    const recentBlockhash = await PROVIDER.connection.getLatestBlockhash();
    const fee = await new Transaction({
      feePayer: PROVIDER.publicKey,
      blockhash: recentBlockhash.blockhash,
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
    })
      .add(await ix.instruction())
      .getEstimatedFee(PROVIDER.connection);
    await ix.rpc({ commitment: "confirmed" });
    const after_balance = await PROVIDER.connection.getBalance(
      PROVIDER.publicKey
    );
    // Only Transaction fee was taken because authority pays non program fees
    expect(
      new BN(before_balance.toString())
        .sub(new BN(after_balance.toString()))
        .toNumber()
    ).to.equal(fee);
  });

  it("Check allowed wallet", async () => {
    const rolePDA = await role_pda(WRITE_PERM.role, ALLOWED_WALLET.publicKey);
    const before_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );
    const ix = PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: rolePDA,
        solCerberusRule: writeRulePDA,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: walletSeedPDA,
        signer: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET]);
    const recentBlockhash = await PROVIDER.connection.getLatestBlockhash();
    const rent_exemption_price =
      await PROVIDER.connection.getMinimumBalanceForRentExemption(9);
    const tx_fee = await new Transaction({
      feePayer: ALLOWED_WALLET.publicKey,
      blockhash: recentBlockhash.blockhash,
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
    })
      .add(await ix.instruction())
      .getEstimatedFee(PROVIDER.connection);
    await ix.rpc({ commitment: "confirmed" });
    const after_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );

    // First "Allowed" check only charges the Rent exemption for the created "Seed account"
    // Note that the transaction fee is payed by the anchor Provider.
    expect(
      new BN(before_balance.toString())
        .sub(new BN(after_balance.toString()))
        .toNumber()
    ).to.equal(rent_exemption_price);

    // Allowed to Read (Applied to all via wildcard)
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
        solCerberusRule: readRulePDA,
        solCerberusToken: null,
        solCerberusMetadata: null,
        solCerberusSeed: walletSeedPDA,
        signer: ALLOWED_WALLET.publicKey,
      })
      .signers([ALLOWED_WALLET])
      .rpc();
    const last_balance = await PROVIDER.connection.getBalance(
      ALLOWED_WALLET.publicKey
    );

    // Posterior checks only cost the Program fee.
    // Note that the transaction fee is payed by the anchor Provider.
    expect(
      new BN(after_balance.toString())
        .sub(new BN(last_balance.toString()))
        .toNumber()
    ).to.equal(FEE);
  });

  it("Check allowed NFT", async () => {
    const metadataPDA = await nft_metadata_pda(NFTS.allowedNFT.mintAddress);
    const rolePDA = await role_pda(
      WRITE_PERM.role,
      NFTS.allowedNFT.mintAddress
    );
    const tokenPDA = await getAssociatedTokenAddress(
      NFTS.allowedNFT.mintAddress,
      WALLET_WITH_NFTS.publicKey
    );
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: rolePDA,
        solCerberusRule: writeRulePDA,
        solCerberusToken: tokenPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusSeed: WalletWithNFTSeedPDA,
        signer: WALLET_WITH_NFTS.publicKey,
      })
      .signers([WALLET_WITH_NFTS])
      .rpc();

    // Allowed to Read (Applied to all via wildcard)
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
        solCerberusRule: readRulePDA,
        solCerberusToken: tokenPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusSeed: WalletWithNFTSeedPDA,
        signer: WALLET_WITH_NFTS.publicKey,
      })
      .signers([WALLET_WITH_NFTS])
      .rpc();

    // Burn the token to verify use has no access anymore
    await burnChecked(
      PROVIDER.connection, // connection
      WALLET_WITH_NFTS, // payer
      NFTS.allowedNFT.tokenAddress, // token account
      NFTS.allowedNFT.mintAddress, // mint
      WALLET_WITH_NFTS.publicKey, // owner
      1, // amount, if your decimals is 9, 10^9 for 1 token
      0
    );
    // Verify user is not allowed anymore
    try {
      await PROGRAM.methods
        .allowed({
          appId: APP_ID,
          namespace: namespaces.Rule,
          resource: WRITE_PERM.resource,
          permission: WRITE_PERM.permission,
        })
        .accounts({
          solCerberusApp: appPDA,
          solCerberusRule: writeRulePDA,
          solCerberusRole: rolePDA,
          solCerberusToken: tokenPDA,
          solCerberusMetadata: metadataPDA,
          solCerberusSeed: WalletWithNFTSeedPDA,
          signer: WALLET_WITH_NFTS.publicKey,
        })
        .signers([WALLET_WITH_NFTS])
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
      WRITE_PERM.role,
      NFTS.allowedCollection.nft.collection.address
    );
    const tokenPDA = await getAssociatedTokenAddress(
      NFTS.allowedCollection.mintAddress,
      WALLET_WITH_NFTS.publicKey
    );
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: WRITE_PERM.resource,
        permission: WRITE_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: rolePDA,
        solCerberusRule: writeRulePDA,
        solCerberusToken: tokenPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusSeed: WalletWithNFTSeedPDA,
        signer: WALLET_WITH_NFTS.publicKey,
      })
      .signers([WALLET_WITH_NFTS])
      .rpc();

    // Allowed to Read (Applied to all via wildcard)
    await PROGRAM.methods
      .allowed({
        appId: APP_ID,
        namespace: namespaces.Rule,
        resource: READ_PERM.resource,
        permission: READ_PERM.permission,
      })
      .accounts({
        solCerberusApp: appPDA,
        solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
        solCerberusRule: readRulePDA,
        solCerberusToken: tokenPDA,
        solCerberusMetadata: metadataPDA,
        solCerberusSeed: WalletWithNFTSeedPDA,
        signer: WALLET_WITH_NFTS.publicKey,
      })
      .signers([WALLET_WITH_NFTS])
      .rpc();
  });
});
