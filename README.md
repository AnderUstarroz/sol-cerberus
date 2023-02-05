# sol-cerberus
Access Control List (ACL) for managing the admission rules on Solana programs


## Requirements
- [Install Rust](https://www.rust-lang.org/tools/install).
- [Install Yarn](https://yarnpkg.com/getting-started/install).
- [Install Anchor](https://book.anchor-lang.com/getting_started/installation.html#anchor).

## Testing
The tests are executed using the **cluster** and **wallet** defined within the `./Anchor.toml` file. To run the tests execute the following command at the root folder of the project:

```
anchor test
```