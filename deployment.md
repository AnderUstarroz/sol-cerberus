# sol-cerberus

## Publish package on crates.io
To Publish The Sol Cerberus package, add the new corresponding version on `programs/sol-cerberus/Cargo.toml` and run:
- `cd programs/sol-cerberus`
- `cargo publish`

## Deploy Sol Cerberus:
Update `Anchor.toml` file defining the `cluster` in which you want to deploy the program, then run:

```
anchor upgrade --program-id SCERbrcgSPwgkrJ7j4TABr17dhYzdgiwPZUSSfFPt8x  target/deploy/sol_cerberus.so
```
## Resuming a failed deployment
When a deployment fails you may get an error similar to this:
```
==================================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
==================================================================================
valley flat great hockey share token excess clever benefit traffic avocado athlete
==================================================================================
To resume a deploy, pass the recovered keypair as
the [BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
==================================================================================
```
To recover the keypair you can run the following command and paste the 12-word seed phrase from the previous error:
```
solana-keygen recover -o buffer-keypair.json
```
Then you can resume the deploy by running:
```
solana program deploy --buffer buffer-keypair.json --url YOUR_RPC_URL target/deploy/sol_cerberus.so --program-id target/deploy/sol_cerberus-keypair.json --keypair PATH_TO_THE_AUTHORITY_KEYPAIR.json
```
## Recover SOL from failed deployments
When a deployment fails you can check the buffer accounts by running
```
solana program show --buffers --keypair PATH_TO_THE_AUTHORITY_KEYPAIR.json
```
To recover the SOL from the buffers just need to run:
```
solana program close --buffers --keypair PATH_TO_THE_AUTHORITY_KEYPAIR.json
```