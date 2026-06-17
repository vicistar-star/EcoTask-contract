# Contributing to EcoTask Contracts

## Prerequisites
- Rust 1.75+
- Soroban CLI (`cargo install --locked soroban-cli`)
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

## Development
```bash
cargo build --target wasm32-unknown-unknown --release
cargo test
cargo clippy -- -D warnings
```

## Deploying
```bash
./scripts/deploy.sh eco-token testnet
```

## Project Structure
- `contracts/eco-token/` — ECO token contract
- `contracts/task-registry/` — Task registry contract
- `contracts/reward-engine/` — Reward engine contract
- `tests/` — Integration tests
- `scripts/` — Deploy and utility scripts
