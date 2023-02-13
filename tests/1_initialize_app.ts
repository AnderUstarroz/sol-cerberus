import * as anchor from "@project-serum/anchor";
import { expect } from "chai";
import { app_pda, safe_airdrop } from "./common";
import {
  APP_KEYPAIR,
  NFTS,
  PROVIDER,
  RECOVERY_KEYPAIR,
  METAPLEX,
  PROGRAM,
  USER,
  PROVIDER_WALLET,
} from "./constants";

describe("1.- Initialize APP", () => {
  let appPDA = null; // Populated on before() block
  const unauthorized_keypair = anchor.web3.Keypair.generate();

  // Create NFTs for testing access rules afterwards.
  before(async () => {
    appPDA = await app_pda();
    await safe_airdrop(
      PROVIDER.connection,
      PROVIDER.wallet.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL // 5 SOL
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
      tokenOwner: USER.publicKey,
      isMutable: true,
    });
    NFTS["allowedCollection"] = await METAPLEX.nfts().create({
      name: "Allowed collection",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft2-hash",
      tokenOwner: USER.publicKey,
      isMutable: true,
      collection: collection.mintAddress,
      collectionAuthority: PROVIDER_WALLET.payer, // This will set the Collection verified flag to true
    });
    NFTS["notAllowedNFT"] = await METAPLEX.nfts().create({
      name: "Not allowed NFT",
      sellerFeeBasisPoints: 0,
      uri: "https://arweave.net/nft3-hash",
      tokenOwner: USER.publicKey,
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
        id: APP_KEYPAIR.publicKey,
        recovery: RECOVERY_KEYPAIR.publicKey,
        name: appName,
      })
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.id.toBase58()).to.equal(APP_KEYPAIR.publicKey.toBase58());
    expect(app.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
    expect(app.name).to.equal(appName);
  });

  it("Update authority", async () => {
    try {
      // Unauthorized users shouldn't be able to update App authority
      await PROGRAM.methods
        .updateAuthority(unauthorized_keypair.publicKey)
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
      .updateAuthority(unauthorized_keypair.publicKey)
      .accounts({
        app: appPDA,
      })
      .rpc();
    let app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.authority.toBase58()).to.equal(
      unauthorized_keypair.publicKey.toBase58()
    );
    // Verify recovery can also update the authority of the APP
    await PROGRAM.methods
      .updateAuthority(PROVIDER.wallet.publicKey)
      .accounts({
        app: appPDA,
        signer: RECOVERY_KEYPAIR.publicKey,
      })
      .signers([RECOVERY_KEYPAIR])
      .rpc();
    app = await PROGRAM.account.app.fetch(appPDA);
    expect(app.authority.toBase58()).to.equal(
      PROVIDER.wallet.publicKey.toBase58()
    );
  });
});
