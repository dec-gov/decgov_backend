#!/usr/bin/env bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo build --target wasm32-unknown-unknown --release

candid-extractor "$SCRIPT_DIR/target/wasm32-unknown-unknown/release/decgov_backend.wasm" > "$SCRIPT_DIR/backend.did"
echo OK
