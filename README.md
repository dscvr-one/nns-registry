# nns-registry

Canister for storing NNS principals via whitelist.

# Development

The canister can be built and deployed using [dfx](https://internetcomputer.org/docs/current/references/cli-reference/dfx-parent).

## Building

Run `dfx build`

## Testing

- Run `dfx start` if needed to run the local replica
- Run `./scripts/run_tests.sh` to run all the unit and integration tests.
  - Change port if needed to point to the local replica
  - The tests use use [ic-repl](https://github.com/chenyan2002/ic-repl) for the integration tests. The test script will automatically install `ic-repl` if it's not present.
  - The test will create canisters
