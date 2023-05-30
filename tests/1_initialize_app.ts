import * as anchor from "@project-serum/anchor";
import { expect, assert } from "chai";
import { app_pda, safe_airdrop } from "./common";
import {
  APP_ID,
  NFTS,
  PROVIDER,
  RECOVERY_KEYPAIR,
  METAPLEX,
  PROGRAM,
  PROVIDER_WALLET,
  WALLET_WITH_NFTS,
  ALLOWED_WALLET,
  ANOTHER_WALLET,
} from "./constants";

describe("1.- Initialize APP", () => {
  let appPDA = null; // Populated on before() block
  const unauthorized_keypair = anchor.web3.Keypair.generate();

  // Create NFTs for testing access rules afterwards.
  before(async () => {
    appPDA = await app_pda();
    PROVIDER.connection.requestAirdrop(
      WALLET_WITH_NFTS.publicKey,
      1_000_000_000 // 1SOL
    );
    await safe_airdrop(
      PROVIDER.connection,
      PROVIDER.wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    // Async airdrop for wallet user
    safe_airdrop(
      PROVIDER.connection,
      ALLOWED_WALLET.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    // Async airdrop for another wallet user
    safe_airdrop(
      PROVIDER.connection,
      ANOTHER_WALLET.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL
    );
    const collection = await METAPLEX.nfts().create({
      name: "Collection",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/collection1-hash",
      isMutable: true,
      isCollection: true,
    });
    NFTS["allowedNFT"] = await METAPLEX.nfts().create({
      name: "Allowed NFT",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft1-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
    });
    NFTS["allowedCollection"] = await METAPLEX.nfts().create({
      name: "Allowed collection",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft2-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
      collection: collection.mintAddress,
      collectionAuthority: PROVIDER_WALLET.payer, // This will set the Collection verified flag to true
    });
    NFTS["notAllowedNFT"] = await METAPLEX.nfts().create({
      name: "Not allowed NFT",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft3-hash",
      tokenOwner: WALLET_WITH_NFTS.publicKey,
      isMutable: true,
    });
  });

  it("Init", async () => {
    const appName = "myapp";
    try {
      await PROGRAM.account.app.fetch(appPDA);
    } catch (_err) {
      expect(_err.toString()).to.include("Account does not exist");
    }
    const tx = await PROGRAM.methods
      .initializeApp({
        id: APP_ID,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: appName,
        cached: false,
      })
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.id.toBase58()).to.equal(APP_ID.toBase58());
    expect(app.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(app.name).to.equal(appName);
  });

  it("Update authority", async () => {
    try {
      // Unauthorized users shouldn't be able to update App authority
      await PROGRAM.methods
        .updateApp({
          authority: unauthorized_keypair.publicKey,
          recovery: RECOVERY_KEYPAIR.publicKey,
          name: "myapp-recovered",
          cached: false,
        })
        .accounts({
          app: appPDA,
          signer: unauthorized_keypair.publicKey,
        })
        .signers([unauthorized_keypair])
        .rpc();
      throw new Error(
        "Unauthorized users shouldn't be able to update App authority!"
      );
    } catch (error) {
      expect(error.error.errorCode.code).to.equal(
        "UnauthorizedAuthorityUpdate"
      );
    }
    // Verify current Authority can update the authority of the APP
    await PROGRAM.methods
      .updateApp({
        authority: unauthorized_keypair.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "myapp-recovered1",
        cached: true,
      })
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.name).to.equal("myapp-recovered1");
    assert.isTrue(app.cached);
    expect(app.authority.toBase58()).to.equal(
      unauthorized_keypair.publicKey.toBase58()
    );
    // Verify recovery can also update the authority of the APP
    await PROGRAM.methods
      .updateApp({
        authority: PROVIDER.wallet.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: "myapp-recovered2",
        cached: false,
      })
      .accounts({
        app: appPDA,
        signer: RECOVERY_KEYPAIR.publicKey,
      })
      .signers([RECOVERY_KEYPAIR])
      .rpc();
    app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.name).to.equal("myapp-recovered2");
    assert.isFalse(app.cached);
    expect(app.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
  });
});
