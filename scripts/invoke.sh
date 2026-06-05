#!/usr/bin/env bash
set -euo pipefail

CONTRACT_ID="${1:?Usage: invoke.sh <contract-id> <function> [args...]}"
FUNCTION="${2:?Usage: invoke.sh <contract-id> <function> [args...]}"
shift 2

echo "Invoking $FUNCTION on $CONTRACT_ID..."
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --network "${NETWORK:-testnet}" \
  --source "$SOROBAN_SECRET_KEY" \
  --fn "$FUNCTION" \
  "$@"
