#!/bin/sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo install --git https://github.com/chenyan2002/ic-repl.git --rev 7913f8b9eb91f66e71b4f5c5d4421fa52e08b97e
dfx identity use default
dfx canister stop --all
dfx canister delete --all
dfx canister create --all
dfx build
dfx canister install --all
NNS_CANISTER_ID=$(dfx canister id nns_registry) \
    ic-repl --replica http://localhost:8000 ${SCRIPT_DIR}/test.ic-repl
