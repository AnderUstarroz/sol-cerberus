## sol-cerberus

## Deploy Sol Cerberus:
Update `Anchor.toml` file defining the `cluster` in which you want to deploy the program, then run:
`anchor upgrade target/deploy/sol_cerberus.so  --program-id SCERbrcgSPwgkrJ7j4TABr17dhYzdgiwPZUSSfFPt8x`

## Publish package on crates.io
To Publish The Sol Cerberus package, add the new corresponding version on `programs/sol-cerberus/Cargo.toml` and run:
- `cd programs/sol-cerberus`
- `cargo publish`