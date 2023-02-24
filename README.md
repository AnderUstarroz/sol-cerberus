# sol-cerberus
A fine grained Access Control List (ACL) for enhanced security on Solana programs.

## Development

### Requirements
- [Install Rust](https://www.rust-lang.org/tools/install).
- [Install Yarn](https://yarnpkg.com/getting-started/install).
- [Install Solana CLI](https://docs.solana.com/es/cli/install-solana-cli-tools).
- [Install Anchor](https://book.anchor-lang.com/getting_started/installation.html#anchor).
- [Install Node v16.19.0](https://www.npmjs.com/package/n).


### Build/update Solana dependencies
The program requires Metaplex program library for testing NFT access. To build/update the dependencies execute the `./Makefile`  by running the following command within the root directory:
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

