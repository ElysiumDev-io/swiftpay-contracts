# SwiftPay Contracts

[![CI](https://github.com/ElysiumDev-io/swiftpay-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/ElysiumDev-io/swiftpay-contracts/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A collection of Soroban smart contracts for the SwiftPay platform on the Stellar network, enabling secure, efficient, and programmable payment solutions.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building Contracts](#building-contracts)
- [Testing](#testing)
- [Deployment](#deployment)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)

## Overview

SwiftPay Contracts provide a suite of smart contracts built on Soroban, Stellar's smart contract platform. These contracts facilitate various payment operations, including direct transfers, escrow services, recurring payments, and multi-party transactions, all while leveraging Stellar's fast, low-cost, and scalable blockchain infrastructure.

The platform aims to revolutionize payment processing by offering programmable money features, enabling developers to build complex financial applications on top of Stellar.

## Features

- **Direct Payments**: Secure and instant token transfers between parties
- **Escrow Services**: Hold funds in escrow until conditions are met
- **Recurring Payments**: Automated subscription-based payment streams
- **Multi-signature Transactions**: Require multiple approvals for high-value transfers
- **Cross-border Payments**: Leverage Stellar's global network for international transfers
- **Programmable Money**: Custom logic for payment conditions and triggers
- **Low Fees**: Benefit from Stellar's minimal transaction costs
- **Fast Finality**: Near-instant transaction confirmations

## Architecture

The SwiftPay ecosystem consists of multiple interconnected smart contracts:

### Core Contracts

- **SwiftPay Main Contract**: Orchestrates payment flows and manages user accounts
- **Escrow Contract**: Handles conditional fund releases
- **Subscription Contract**: Manages recurring payment schedules
- **MultiSig Contract**: Implements multi-signature approval mechanisms

### Design Principles

- **Modularity**: Each contract serves a specific purpose, allowing for flexible composition
- **Security First**: Extensive use of access controls, reentrancy guards, and formal verification
- **Upgradeability**: Proxy patterns for seamless contract updates
- **Interoperability**: Standards-compliant interfaces for integration with other Stellar dApps

## Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Soroban CLI**: For contract development and deployment
- **Stellar Account**: With sufficient XLM for fees and storage

## Installation

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Soroban CLI**:
   ```bash
   cargo install soroban-cli
   ```

3. **Clone the repository**:
   ```bash
   git clone https://github.com/ElysiumDev-io/swiftpay-contracts.git
   cd swiftpay-contracts
   ```

## Building Contracts

Build all contracts in the workspace:

```bash
# Build all contracts
for contract in contracts/*/; do
  if [ -d "$contract" ]; then
    cd "$contract"
    soroban contract build
    cd -
  fi
done
```

This will generate WebAssembly (WASM) binaries in each contract's `target/wasm32-unknown-unknown/release/` directory.

## Testing

Run the test suite for all contracts:

```bash
# Test all contracts
for contract in contracts/*/; do
  if [ -d "$contract" ]; then
    cd "$contract"
    cargo test
    cd -
  fi
done
```

For integration testing with the Soroban environment:

```bash
soroban contract test --wasm target/wasm32-unknown-unknown/release/swiftpay.wasm
```

## Deployment

### Local Deployment (for development)

1. Start a local Soroban network:
   ```bash
   soroban dev
   ```

2. Deploy contracts:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/swiftpay.wasm \
     --source alice
   ```

### Testnet Deployment

1. Configure for testnet:
   ```bash
   soroban config network add testnet \
     --rpc-url https://soroban-testnet.stellar.org \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

2. Fund your account with test XLM from the [Stellar Testnet Faucet](https://developers.stellar.org/docs/fundamentals/networks/testnet#faucet)

3. Deploy:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/swiftpay.wasm \
     --source <your-secret-key> \
     --network testnet
   ```

### Mainnet Deployment

⚠️ **Caution**: Mainnet deployments are permanent and costly. Ensure thorough testing before deploying.

1. Configure for mainnet:
   ```bash
   soroban config network add mainnet \
     --rpc-url https://soroban.stellar.org \
     --network-passphrase "Public Global Stellar Network ; September 2015"
   ```

2. Deploy with sufficient XLM for fees.

## API Documentation

### SwiftPay Contract

#### `init(env: Env, admin: Address)`

Initializes the contract with an admin address.

#### `transfer(env: Env, from: Address, to: Address, amount: i128)`

Transfers tokens from one address to another.

#### `escrow_create(env: Env, buyer: Address, seller: Address, amount: i128, deadline: u64)`

Creates an escrow agreement between buyer and seller.

#### `escrow_release(env: Env, escrow_id: u64)`

Releases funds from escrow to the seller.

#### `subscription_create(env: Env, subscriber: Address, merchant: Address, amount: i128, interval: u64)`

Creates a recurring payment subscription.

For complete API documentation, see the [docs/](docs/) directory or inline code comments.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment
- Code style guidelines
- Testing requirements
- Pull request process

## Security

Security is paramount in financial smart contracts. Please report any security vulnerabilities to [security@swiftpay.io](mailto:security@swiftpay.io) before disclosing publicly.

See our [Security Policy](SECURITY.md) for more information.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.


Built with ❤️ on the Stellar network using Soroban.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
