import { BN } from "bn.js";
import { expect } from "chai";
import { app_pda, role_pda, READ_PERM } from "./common";
import {
  addressType,
  APP_ID,
  NFTS,
  PROGRAM,
  USER_ALLOWED_WALLET,
} from "./constants";

describe("3.- Assign roles", () => {
  let appPDA = null; // Populated on before() block

  before(async () => {
    appPDA = await app_pda();
  });
  it("Assign role to NFT", async () => {
    const rolePDA = await role_pda(READ_PERM.role, NFTS.allowedNFT.mintAddress);
    const oneHourLater = Math.floor(new Date().getTime() / 1000) + 60 * 60;
    let listener = null;
    let [event, _]: any = await new Promise((resolve, _reject) => {
      listener = PROGRAM.addEventListener("RolesChanged", (event, slot) => {
        PROGRAM.removeEventListener(listener);
        resolve([event, slot]);
      });
      PROGRAM.methods
        .assignRole({
          address: NFTS.allowedNFT.mintAddress,
          role: READ_PERM.role,
          addressType: addressType.NFT,
          expiresAt: new BN(oneHourLater),
        })
        .accounts({
          app: appPDA,
          role: rolePDA,
        })
        .rpc();
    });
    const role = await PROGRAM.account.role.fetch(rolePDA);
    expect(APP_ID.toBase58()).to.equal(event.appId.toBase58());
    expect(role.address.toBase58()).to.equal(
      NFTS.allowedNFT.mintAddress.toBase58()
    );
    expect(role.role).to.equal(READ_PERM.role);
    expect(role.addressType).to.deep.equal(addressType.NFT);
    expect(role.expiresAt.toNumber()).to.equal(oneHourLater);
  });

  it("Assign role to NFT Collection", async () => {
    const rolePDA = await role_pda(
      READ_PERM.role,
      NFTS.allowedCollection.nft.collection.address
    );
    await PROGRAM.methods
      .assignRole({
        address: NFTS.allowedCollection.nft.collection.address,
        role: READ_PERM.role,
        addressType: addressType.Collection,
        expiresAt: null,
      })
      .accounts({
        app: appPDA,
        role: rolePDA,
      })
      .rpc();
  });

  it("Assign role to Wallet", async () => {
    const rolePDA = await role_pda(
      READ_PERM.role,
      USER_ALLOWED_WALLET.publicKey
    );
    await PROGRAM.methods
      .assignRole({
        address: USER_ALLOWED_WALLET.publicKey,
        role: READ_PERM.role,
        addressType: addressType.Wallet,
        expiresAt: null,
      })
      .accounts({
        app: appPDA,
        role: rolePDA,
      })
      .rpc();
  });
});
