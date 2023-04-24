#!/usr/bin/env bash
set -euo pipefail

dfx canister create --no-wallet backend

dfx build backend
gzip -f -1 ./target/wasm32-unknown-unknown/release/backend.wasm

cargo test

dfx canister install backend --argument "(record {
  known_principal_ids = vec {
    record {
      variant { UserIdGlobalSuperAdmin };
      principal \"$(dfx identity get-principal)\";
    };
    record {
      variant { CanisterIdConfiguration };
      principal \"$(dfx canister id backend)\";
    };
  };
})" --wasm target/wasm32-unknown-unknown/release/backend.wasm.gz