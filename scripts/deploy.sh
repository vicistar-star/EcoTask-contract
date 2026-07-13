#!/usr/bin/env bash
set -euo pipefail

CONTRACT_NAME="${1:?Usage: deploy.sh <contract-name> <network>}"
NETWORK="${2:-testnet}"
WASM="target/wasm32v1-none/release/${CONTRACT_NAME//-/_}.wasm"

if [ ! -f "$WASM" ]; then
  echo "Error: WASM file not found at $WASM"
  echo "Build first: cargo build --target wasm32v1-none --release -p $CONTRACT_NAME"
  exit 1
fi

echo "Deploying $CONTRACT_NAME to $NETWORK..."
soroban contract deploy \
  --wasm "$WASM" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY"
