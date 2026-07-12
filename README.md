<div align="center">

# 🔗 ecotask-contracts

**The on-chain heart of EcoTask — smart contracts powering verifiable climate rewards.**

*Stellar Soroban contracts written in Rust that handle token issuance, task registration, and trustless reward distribution.*

[![Build](https://img.shields.io/badge/Build-Passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/Rust-1.75-orange?logo=rust)](https://www.rust-lang.org)
[![Soroban](https://img.shields.io/badge/Soroban-Smart%20Contracts-7B68EE?logo=stellar)](https://soroban.stellar.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/Status-v0.1.0--alpha-blue)]()

</div>

---

## 🌍 Overview

`ecotask-contracts` contains the Soroban smart contracts that power the trustless, transparent reward system at the core of EcoTask.

These contracts run on the **Stellar blockchain** and are responsible for:

- 🪙 Issuing and managing the **ECO token**
- 📋 Registering tasks and their reward parameters on-chain
- 🔍 Processing verifications and releasing rewards automatically
- 🗳️ Future DAO governance for platform decisions

Every reward payout is **transparent, auditable, and trustless** — no middleman, no delays, no corruption.

---

## 📦 Contracts

### 1. `eco-token`
The EcoTask native token contract.

- Issues ECO tokens tied to verified environmental impact
- Controls minting — only the reward engine can mint new tokens
- Implements the Stellar token interface (SEP-0041 compatible)
- Supports token metadata: name, symbol, decimals

### 2. `task-registry`
The on-chain task database.

- Stores task definitions: type, location hash, reward amount, expiry
- Controls who can create tasks (admins, verified NGOs, sponsors)
- Emits events when tasks are created, completed, or expired
- Prevents double-claiming — tracks which wallets completed which tasks

### 3. `reward-engine`
The verification and payout engine.

- Receives verification results from the off-chain oracle
- Validates proof hashes against IPFS CIDs stored at submission
- Mints ECO tokens or transfers USDC to the user's wallet on success
- Handles disputes and partial rewards for incomplete tasks

---

## 🚀 Quick Start for Contributors

Get up and running in minutes:

1.  **Install Prerequisites**:
    ```bash
    rustup target add wasm32-unknown-unknown
    cargo install --locked soroban-cli
    ```
2.  **Clone & Build**:
    ```bash
    git clone https://github.com/ecotask/ecotask-contracts.git
    cd ecotask-contracts
    make build
    ```
3.  **Run Tests**:
    ```bash
    make test
    ```

For more details, see [CONTRIBUTING.md](CONTRIBUTING.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) for more information on how to get involved.

---

## 🏗️ Folder Structure

```
ecotask-contracts/
├── contracts/
│   ├── eco-token/
│   │   ├── src/
│   │   │   ├── lib.rs            # Contract entry point
│   │   │   ├── token.rs          # Token logic (mint, transfer, burn, approve)
│   │   │   └── storage.rs        # On-chain state management
│   │   └── Cargo.toml
│   │
│   ├── task-registry/
│   │   ├── src/
│   │   │   ├── lib.rs            # Contract entry point
│   │   │   ├── registry.rs       # Task CRUD operations
│   │   │   ├── access.rs         # Role-based access control
│   │   │   └── storage.rs        # On-chain state management
│   │   └── Cargo.toml
│   │
│   └── reward-engine/
│       ├── src/
│       │   ├── lib.rs            # Contract entry point
│       │   ├── verification.rs   # Proof validation and reward logic
│       │   └── storage.rs        # On-chain state management
│       ├── tests/
│       │   └── full_lifecycle_test.rs  # Cross-contract integration tests
│       └── Cargo.toml
│
├── scripts/
│   ├── deploy.sh                 # Deploy contracts to testnet/mainnet
│   ├── invoke.sh                 # Helper to call contract functions
│   ├── fund-accounts.sh          # Fund test accounts with friendbot
│   ├── verify-deploy.sh          # Verify deployed contract state
│   └── integration-test.sh       # End-to-end integration test runner
│
├── .github/
│   └── workflows/
│       └── ci.yml                # CI pipeline (build, test, lint, fmt)
│
├── Cargo.toml                    # Workspace config
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) >= 1.75
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- A Stellar testnet account (funded via [Friendbot](https://laboratory.stellar.org/#account-creator))

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Add the WebAssembly target
rustup target add wasm32-unknown-unknown
```

### Build

```bash
# Clone the repo
git clone https://github.com/ecotask-network/ecotask-contracts.git
cd ecotask-contracts

# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Build a specific contract
cd contracts/eco-token
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
# Run all tests
cargo test

# Run tests for a specific contract
cargo test -p eco-token
cargo test -p task-registry
cargo test -p reward-engine
```

### Deploy to Testnet

```bash
# Configure Soroban CLI for testnet
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Deploy the ECO token contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/eco_token.wasm \
  --network testnet \
  --source YOUR_SECRET_KEY

# Note the contract ID and add to your .env
```

---

## 🔐 Contract Architecture

```
                        ┌─────────────────┐
                        │   Task Registry  │
                        │                 │
                        │ • Store tasks   │
                        │ • Track claims  │
                        └────────┬────────┘
                                 │ task data
                                 ▼
User submits proof ──▶  ┌─────────────────┐     ┌──────────────┐
(off-chain oracle)      │  Reward Engine  │────▶│  ECO Token   │
                        │                 │mint  │              │
                        │ • Verify proof  │     │ • Mint ECO   │
                        │ • Check task    │     │ • Transfer   │
                        │ • Release pay   │     └──────────────┘
                        └─────────────────┘
                                 │ USDC transfer
                                 ▼
                        ┌─────────────────┐
                        │  User Wallet    │
                        │  (Stellar)      │
                        └─────────────────┘
```

---

## 🧪 Example: Calling the Reward Engine

```bash
# Invoke the reward engine to process a verified task
soroban contract invoke \
  --id YOUR_REWARD_ENGINE_CONTRACT_ID \
  --network testnet \
  --source YOUR_SECRET_KEY \
  -- \
  process_reward \
  --user GUSER_PUBLIC_KEY \
  --task_id "task_001" \
  --proof_cid "QmXyz...ipfs_hash" \
  --reward_amount 100
```

---

## 🔒 Security

- All contracts are designed for formal audit before mainnet deployment
- Minting is restricted to the reward engine contract only
- Task creation requires an admin or verified sponsor signature
- Proof hashes are stored at submission time to prevent retroactive fraud
- See [SECURITY.md](./SECURITY.md) to report vulnerabilities

---

## 🤝 Contributing

Rust and Soroban experience helpful but not required — we're happy to mentor.
See [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

---

## 📄 License

MIT — see [LICENSE](./LICENSE) for details.

---

## Ecosystem

This is part of the [EcoTask Network](https://github.com/ecotask-network):

| Repo | Description |
|------|-------------|
| [EcoTask-app](https://github.com/ecotask-network/EcoTask-app) | Mobile dApp |
| [EcoTask-backend](https://github.com/ecotask-network/EcoTask-backend) | Node.js API & verification engine |
| [EcoTask-contracts](https://github.com/ecotask-network/EcoTask-contract) | Stellar Soroban smart contracts |
| [EcoTask-docs](https://github.com/ecotask-network/EcoTask-docs) | Documentation hub |

---

<div align="center">

*Part of the [EcoTask Network](https://github.com/ecotask-network) — Because the environment deserves an economy.*

</div>
