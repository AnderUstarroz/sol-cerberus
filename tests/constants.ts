import * as anchor from "@project-serum/anchor";

export const APP_KEYPAIR = anchor.web3.Keypair.generate();
export const RECOVERY_KEYPAIR = anchor.web3.Keypair.generate();

// @TODO Improve this hack:
// NFTs are created on Step 0 populating the following object, so they can be used on all other tests.
export const NFTS = {
  nft_allowed: null,
  collection_allowed: null,
  not_allowed: null,
};
