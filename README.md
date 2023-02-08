# sol-cerberus
Access Control List (ACL) for managing the admission rules on Solana programs


## Requirements
- [Install Rust](https://www.rust-lang.org/tools/install).
- [Install Yarn](https://yarnpkg.com/getting-started/install).
- [Install Anchor](https://book.anchor-lang.com/getting_started/installation.html#anchor).
- [Install Node v16.19.0](https://www.npmjs.com/package/n).

## Development

### Add Solana dependencies (only the first one you setup the project)
The program requires Metaplex program library for testing NFT access. Run the following command from root folder to create the required dependency.
```
git submodule add https://github.com/metaplex-foundation/metaplex-program-library deps/metaplex-program-library
```

### Build/update Solana dependencies
To build/update the Solana dependencies execute the `./Makefile`  by running the following command within the root directory:
```
make
```
## Testing
The tests are executed using the **cluster** and **wallet** defined within the `./Anchor.toml` file.

### Run tests
To run the tests execute the following command at the root folder of the project:

```
anchor test
```



PENDING:


How to add external Program to your test suit:
https://www.anchor-lang.com/docs/manifest#test
https://github.com/coral-xyz/xnft/blob/0.2.0/Anchor.toml#L17-L19
https://github.com/coral-xyz/xnft/blob/0.2.0/.gitmodules


Verificar NFT:
https://github.com/rhribar/solana-development/blob/a7473769e6ee58e6ad188c977604302f522f2689/non-custodial-escrow/programs/non-custodial-escrow/src/lib.rs
https://github.com/smile930307/solana-arcryptian-nft-breeding/blob/9997d958a7f7f9c82ba32c3c8113100991b06ee4/programs/arcryptiannft-breeding-solana/src/contexts.rs

Verificar Collection:
https://solana.stackexchange.com/questions/5613/how-to-get-the-collections-public-key-out-of-certain-nft-in-anchor/5631#5631