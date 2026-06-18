# Contributing to EcoTask Contracts

First off, thank you for considering contributing to EcoTask! It's people like you that make EcoTask a great tool for the environmental community.

## Table of Contents

- [Prerequisites](#prerequisites)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Development Workflow](#development-workflow)
- [Style Guide](#style-guide)
- [Security](#security)

## Prerequisites

To build and test the smart contracts, you will need:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version, at least 1.75+)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please create an issue using the Bug Report template. Include as much detail as possible, such as:

- Steps to reproduce the bug.
- Expected behavior vs actual behavior.
- Version of Rust and Soroban CLI you are using.

### Suggesting Enhancements

Enhancement suggestions are welcome! Please open an issue using the Feature Request template and describe:

- The problem this enhancement solves.
- A clear and concise description of the proposed change.

### Pull Requests

1. Fork the repository and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes (`cargo test`).
4. Format your code (`cargo fmt`).
5. Lint your code (`cargo clippy`).
6. Update the documentation if necessary.
7. Submit a pull request.

## Development Workflow

### Building

Build all contracts to WASM:

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Testing

Run the full test suite:

```bash
cargo test
```

### Linting & Formatting

We maintain strict linting rules. Ensure your code is clean before submitting:

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Style Guide

- Follow standard [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html).
- Keep functions small and focused.
- Document public functions and complex logic using Doc comments (`///`).
- Soroban-specific: Be mindful of ledger footprint and CPU cycles.

## Security

If you discover a security vulnerability, please do NOT open a public issue. Instead, email us at security@ecotask.network.

---

Thank you for your contributions! 🌍
