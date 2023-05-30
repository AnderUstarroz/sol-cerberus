// import {
//   app_pda,
//   role_pda,
//   nft_metadata_pda,
//   WRITE_PERM,
//   rule_pda,
//   READ_PERM,
// } from "./common";
// import {
//   APP_ID,
//   NFTS,
//   PROGRAM,
//   PROVIDER,
//   ALLOWED_WALLET,
//   WALLET_WITH_NFTS,
//   namespaces,
// } from "./constants";
// import { getAssociatedTokenAddress } from "@solana/spl-token";
// import { burnChecked } from "@solana/spl-token";
// import { expect } from "chai";

// describe("4.- Check permissions", () => {
//   let appPDA = null; // Populated on before() block
//   let writeRulePDA = null; // Populated on before() block
//   let readRulePDA = null; // Populated on before() block

//   before(async () => {
//     appPDA = await app_pda();
//     writeRulePDA = await rule_pda(
//       WRITE_PERM.role,
//       WRITE_PERM.resource,
//       WRITE_PERM.permission
//     );
//     readRulePDA = await rule_pda(
//       READ_PERM.role,
//       READ_PERM.resource,
//       READ_PERM.permission
//     );
//   });
//   it("Check allowed Authority", async () => {
//     // Allowed to Write
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: WRITE_PERM.resource,
//         permission: WRITE_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusToken: null,
//         solCerberusMetadata: null,
//         solCerberusRule: writeRulePDA,
//         solCerberusRole: null,
//       })
//       .rpc();
//   });

//   it("Check allowed NFT", async () => {
//     const metadataPDA = await nft_metadata_pda(NFTS.allowedNFT.mintAddress);
//     const rolePDA = await role_pda(
//       WRITE_PERM.role,
//       NFTS.allowedNFT.mintAddress
//     );
//     const tokenPDA = await getAssociatedTokenAddress(
//       NFTS.allowedNFT.mintAddress,
//       WALLET_WITH_NFTS.publicKey
//     );
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: WRITE_PERM.resource,
//         permission: WRITE_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: rolePDA,
//         solCerberusRule: writeRulePDA,
//         solCerberusToken: tokenPDA,
//         solCerberusMetadata: metadataPDA,
//         signer: WALLET_WITH_NFTS.publicKey,
//       })
//       .signers([WALLET_WITH_NFTS])
//       .rpc();

//     // Allowed to Read (Applied to all via wildcard)
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: READ_PERM.resource,
//         permission: READ_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
//         solCerberusRule: readRulePDA,
//         solCerberusToken: tokenPDA,
//         solCerberusMetadata: metadataPDA,
//         signer: WALLET_WITH_NFTS.publicKey,
//       })
//       .signers([WALLET_WITH_NFTS])
//       .rpc();

//     // Burn the token to verify use has no access anymore
//     await burnChecked(
//       PROVIDER.connection, // connection
//       WALLET_WITH_NFTS, // payer
//       NFTS.allowedNFT.tokenAddress, // token account
//       NFTS.allowedNFT.mintAddress, // mint
//       WALLET_WITH_NFTS.publicKey, // owner
//       1, // amount, if your decimals is 9, 10^9 for 1 token
//       0
//     );
//     // Verify user is not allowed anymore
//     try {
//       await PROGRAM.methods
//         .allowed({
//           appId: APP_ID,
//           namespace: namespaces.Rule,
//           resource: WRITE_PERM.resource,
//           permission: WRITE_PERM.permission,
//         })
//         .accounts({
//           solCerberusApp: appPDA,
//           solCerberusToken: tokenPDA,
//           solCerberusMetadata: metadataPDA,
//           solCerberusRule: writeRulePDA,
//           solCerberusRole: rolePDA,
//           signer: WALLET_WITH_NFTS.publicKey,
//         })
//         .signers([WALLET_WITH_NFTS])
//         .rpc();
//       throw new Error(
//         "NFT authenticated users should not be able to access after burning their NFT"
//       );
//     } catch (e) {
//       if (!e.hasOwnProperty("error")) {
//         throw e;
//       }
//       expect(e.error.errorCode.code).to.equal("Unauthorized");
//     }
//   });

//   it("Check allowed NFT collection", async () => {
//     const metadataPDA = await nft_metadata_pda(
//       NFTS.allowedCollection.mintAddress
//     );
//     const rolePDA = await role_pda(
//       WRITE_PERM.role,
//       NFTS.allowedCollection.nft.collection.address
//     );
//     const tokenPDA = await getAssociatedTokenAddress(
//       NFTS.allowedCollection.mintAddress,
//       WALLET_WITH_NFTS.publicKey
//     );
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: WRITE_PERM.resource,
//         permission: WRITE_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: rolePDA,
//         solCerberusRule: writeRulePDA,
//         solCerberusToken: tokenPDA,
//         solCerberusMetadata: metadataPDA,
//         signer: WALLET_WITH_NFTS.publicKey,
//       })
//       .signers([WALLET_WITH_NFTS])
//       .rpc();

//     // Allowed to Read (Applied to all via wildcard)
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: READ_PERM.resource,
//         permission: READ_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
//         solCerberusRule: readRulePDA,
//         solCerberusToken: tokenPDA,
//         solCerberusMetadata: metadataPDA,
//         signer: WALLET_WITH_NFTS.publicKey,
//       })
//       .signers([WALLET_WITH_NFTS])
//       .rpc();
//   });

//   it("Check allowed wallet", async () => {
//     const rolePDA = await role_pda(WRITE_PERM.role, ALLOWED_WALLET.publicKey);
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: WRITE_PERM.resource,
//         permission: WRITE_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: rolePDA,
//         solCerberusRule: writeRulePDA,
//         solCerberusToken: null,
//         solCerberusMetadata: null,
//         signer: ALLOWED_WALLET.publicKey,
//       })
//       .signers([ALLOWED_WALLET])
//       .rpc();

//     // Allowed to Read (Applied to all via wildcard)
//     await PROGRAM.methods
//       .allowed({
//         appId: APP_ID,
//         namespace: namespaces.Rule,
//         resource: READ_PERM.resource,
//         permission: READ_PERM.permission,
//       })
//       .accounts({
//         solCerberusApp: appPDA,
//         solCerberusRole: await role_pda(READ_PERM.role, null), // Null address represents the wildcard "*"
//         solCerberusRule: readRulePDA,
//         solCerberusToken: null,
//         solCerberusMetadata: null,
//         signer: ALLOWED_WALLET.publicKey,
//       })
//       .signers([ALLOWED_WALLET])
//       .rpc();
//   });
// });
