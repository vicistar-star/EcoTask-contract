#!/usr/bin/env bash
set -euo pipefail

echo "=== EcoTask Contract Integration Test ==="

echo "Creating task..."
soroban contract invoke \
  --id "$TASK_REGISTRY_ID" \
  --network testnet \
  --source "$SPONSOR_KEY" \
  -- \
  create_task \
  --creator "$SPONSOR_ADDRESS" \
  --task_type "{\"string\": \"TREE_PLANTING\"}" \
  --location_hash "$(echo -n 'test' | xxd -p -c 32)" \
  --reward_amount 100 \
  --max_completions 10 \
  --expires_at 9999999999

echo "Task count:"
soroban contract invoke \
  --id "$TASK_REGISTRY_ID" \
  --network testnet \
  --source "$SPONSOR_KEY" \
  -- \
  task_count

echo "=== Integration test complete ==="
