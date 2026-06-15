#!/usr/bin/env bash
set -euo pipefail
NETWORK="testnet"
echo "Verifying contracts on $NETWORK..."

soroban contract invoke \
  --id "$ECO_TOKEN_ID" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY" \
  -- \
  name

soroban contract invoke \
  --id "$ECO_TOKEN_ID" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY" \
  -- \
  symbol

soroban contract invoke \
  --id "$ECO_TOKEN_ID" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY" \
  -- \
  decimal

soroban contract invoke \
  --id "$TASK_REGISTRY_ID" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY" \
  -- \
  task_count

soroban contract invoke \
  --id "$REWARD_ENGINE_ID" \
  --network "$NETWORK" \
  --source "$SOROBAN_SECRET_KEY" \
  -- \
  get_verification \
  --task_id 0 \
  --user "$TEST_USER"

echo "All contracts verified"
