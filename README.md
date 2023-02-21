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

