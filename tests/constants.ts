import * as anchor from "@project-serum/anchor";
import { SolCerberus } from "../target/types/sol_cerberus";
import { keypairIdentity, Metaplex } from "@metaplex-foundation/js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";

export const PROGRAM = anchor.workspace
  .SolCerberus as anchor.Program<SolCerberus>;
export const PROVIDER = anchor.AnchorProvider.env();
export const PROVIDER_WALLET = (PROGRAM.provider as anchor.AnchorProvider)
  .wallet as NodeWallet;
export const METAPLEX = new Metaplex(PROGRAM.provider.connection).use(
  keypairIdentity(PROVIDER_WALLET.payer)
);
export const USER = anchor.web3.Keypair.generate();
export const APP_KEYPAIR = anchor.web3.Keypair.generate();
export const RECOVERY_KEYPAIR = anchor.web3.Keypair.generate();
export const METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
// @TODO Improve this hack:
// NFTs are created on Step 0 populating the following object, so they can be used on all other tests.
export const NFTS: {
  allowedNFT: any;
  allowedCollection: any;
  notAllowedNFT: any;
} = {
  allowedNFT: null,
  allowedCollection: null,
  notAllowedNFT: null,
};

export const addressType: any = {
  Wallet: { wallet: {} },
  NFT: { nft: {} },
  Collection: { collection: {} },
};
