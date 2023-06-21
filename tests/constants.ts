import * as anchor from "@project-serum/anchor";
import { SolCerberus as SolCerberusTypes } from "../target/types/sol_cerberus";
// import { TestCpi } from "../target/types/test_cpi";
import { keypairIdentity, Metaplex } from "@metaplex-foundation/js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";

export const PROGRAM = anchor.workspace
  .SolCerberus as anchor.Program<SolCerberusTypes>;
export const PROVIDER = anchor.AnchorProvider.env();
export const PROVIDER_WALLET = (PROGRAM.provider as anchor.AnchorProvider)
  .wallet as NodeWallet;
export const METAPLEX = new Metaplex(PROGRAM.provider.connection).use(
  keypairIdentity(PROVIDER_WALLET.payer)
);
export const ALLOWED_WALLET: anchor.web3.Keypair =
  anchor.web3.Keypair.generate();

export const WALLET_WITH_NFTS: anchor.web3.Keypair =
  anchor.web3.Keypair.generate();

export const ANOTHER_WALLET: anchor.web3.Keypair =
  anchor.web3.Keypair.generate();

export const APP_ID = new anchor.web3.PublicKey(
  "k39mmqs9rEJ6BMvSF84yw91qnRBXKEBJbeQnZVX7NC3"
);
export const RECOVERY_KEYPAIR = anchor.web3.Keypair.generate();
export const METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export enum namespaces {
  Rule = 0,
  AssignRole = 1,
  DeleteAssignRole = 2,
  AddRuleNSRole = 3,
  AddRuleResourcePerm = 4,
  DeleteRuleNSRole = 5,
  DeleteRuleResourcePerm = 6,
}

export enum classes {
  Trial = 0,
  Free = 1,
}

// @TODO Improve this hack:
// NFTs are created on Step 0 populating the following object, so they can be reused on all other tests.
export const NFTS: {
  allowedNFT: any;
  allowedCollection: any;
  notAllowedNFT: any;
} = {
  allowedNFT: null,
  allowedCollection: null,
  notAllowedNFT: null,
};

export const addressType = {
  Wallet: { wallet: {} },
  Nft: { nft: {} },
  Collection: { collection: {} },
};

export const FEE = 5000;

type AddressKeysType = keyof typeof addressType;
