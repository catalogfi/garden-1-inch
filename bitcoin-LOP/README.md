# Bitcoin Order Protocol

A comprehensive Bitcoin smart contract on the Internet Computer that implements a decentralized order protocol with HTLC (Hash Time-Locked Contract) functionality. This project enables secure peer-to-peer Bitcoin transactions with time-locked escrow mechanisms and supports multiple Bitcoin address types.

Built for the 1inch Hackathon by the Catalog team.

## Table of contents

* [Architecture](#architecture)
* [Features](#features)
* [Order Protocol](#order-protocol)
* [Building and deploying the smart contract locally](#building-and-deploying-the-smart-contract-locally)
  * [1. Prerequisites](#1-prerequisites)
  * [2. Clone the repository](#2-clone-the-repository)
  * [3. Start the ICP execution environment](#3-start-the-icp-execution-environment)
  * [4. Start Bitcoin regtest](#4-start-bitcoin-regtest)
  * [5. Deploy the smart contract](#5-deploy-the-smart-contract)
* [Basic Bitcoin Operations](#basic-bitcoin-operations)
  * [Checking balance](#checking-balance)
  * [Getting UTXOs](#getting-utxos)
  * [Fee estimation](#fee-estimation)
* [Order Protocol Usage](#order-protocol-usage)
  * [Creating orders](#creating-orders)
  * [Viewing orders](#viewing-orders)
  * [Executing order withdrawals](#executing-order-withdrawals)
* [Security considerations](#security-considerations)

  * [Prerequisites for Bitcoin assets](#prerequisites-for-bitcoin-assets)
* [Inscribe an Ordinal](#inscribe-an-ordinal)
* [Etch a Rune](#etch-a-rune)
* [Deploy a BRC-20 token](#deploy-a-brc-20-token)
* [Notes on implementation](#notes-on-implementation)
* [Security considerations and best practices](#security-considerations-and-best-practices)

## Architecture

The Bitcoin Order Protocol integrates with the Internet Computer's built-in APIs to provide a secure and decentralized order management system:

* [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key) - For Bitcoin signature generation
* [Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr) - For Taproot transactions
* [Bitcoin API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md) - For Bitcoin network integration

For background on the ICP<>BTC integration, refer to the [Learn Hub](https://learn.internetcomputer.org/hc/en-us/articles/34211154520084-Bitcoin-Integration).

## Features

🚀 **Core Functionality:**
- **Order Protocol**: Create and manage Bitcoin orders with HTLC escrow
- **Multi-address Support**: Compatible with P2PKH, P2WPKH, and P2TR address types
- **Time-locked Contracts**: Secure transactions with configurable time locks
- **Fee Estimation**: Dynamic Bitcoin network fee calculation
- **UTXO Management**: Efficient unspent transaction output handling

🔒 **Security Features:**
- **Hash Time-Locked Contracts (HTLC)**: Trustless order execution
- **Secret Hash Verification**: Cryptographic proof of payment
- **Time-lock Protection**: Automatic refund mechanisms
- **Multi-signature Support**: Enhanced transaction security

## Order Protocol

The Bitcoin Order Protocol enables secure peer-to-peer transactions through a sophisticated escrow system:

1. **Order Creation**: Initiator creates an order with time lock and secret hash
2. **Order Address Generation**: Unique Bitcoin address generated for each order
3. **Funding**: Initiator funds the order address with Bitcoin
4. **Response**: Responder can preview and execute withdrawal to HTLC
5. **Settlement**: Cryptographic proof enables final settlement or time-lock refund


## Deploying from ICP Ninja

This project can be deployed directly to the Internet Computer using ICP Ninja, where it connects to Bitcoin **testnet4**. Note: Canisters deployed via ICP Ninja remain live for 50 minutes after signing in with your Internet Identity.

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/SurajNaidu0/bitcoin_order_protocol)

## Building and deploying the smart contract locally

### 1. Prerequisites

* [x] [Rust toolchain](https://www.rust-lang.org/tools/install)
* [x] [Internet Computer SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
* [x] [Local Bitcoin testnet (regtest)](https://internetcomputer.org/docs/build-on-btc/btc-dev-env#create-a-local-bitcoin-testnet-regtest-with-bitcoind)
* [x] On macOS, an `llvm` version that supports the `wasm32-unknown-unknown` target is required. The Rust `bitcoin` library relies on the `secp256k1-sys` crate, which requires `llvm` to build. The default `llvm` version provided by XCode does not meet this requirement. Install the [Homebrew version](https://formulae.brew.sh/formula/llvm) using `brew install llvm`.


### 2. Clone the repository

```bash
git clone https://github.com/SurajNaidu0/bitcoin_order_protocol.git
cd bitcoin_order_protocol
```

### 3. Start the ICP execution environment


Open a terminal window (terminal 1) and run the following:
```bash
dfx start --enable-bitcoin --bitcoin-node 127.0.0.1:18444
```
This starts a local canister execution environment with Bitcoin support enabled.

### 4. Start Bitcoin regtest

Open another terminal window (terminal 2) and run the following to start the local Bitcoin regtest network:

```bash
bitcoind -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/bitcoin_data --port=18444
```

### 5. Deploy the smart contract

Open a third terminal (terminal 3) and run the following to deploy the smart contract:

```bash
dfx deploy basic_bitcoin --argument '(variant { regtest })'
```

What this does:

- `dfx deploy` tells the command line interface to `deploy` the smart contract.
- `--argument '(variant { regtest })'` passes the argument `regtest` to initialize the smart contract, telling it to connect to the local Bitcoin regtest network.

Your Bitcoin Order Protocol smart contract is live and ready to use! You can interact with it using either the command line or the Candid UI (the link you see in the terminal).
## Basic Bitcoin Operations

The Bitcoin Order Protocol provides essential Bitcoin functionality to support order operations:

### Checking balance

Check the balance of any Bitcoin address:
```bash
dfx canister call basic_bitcoin get_balance '("<bitcoin_address>")'
```

This uses `bitcoin_get_balance` and works for any supported address type. The balance requires at least one confirmation to be reflected.

### Getting UTXOs

Retrieve unspent transaction outputs for an address:
```bash
dfx canister call basic_bitcoin get_utxos '("<bitcoin_address>")'
```

### Fee estimation

Get current Bitcoin network fee percentiles:
```bash
dfx canister call basic_bitcoin get_current_fee_percentiles '()'
```

## Order Protocol Usage

The core functionality of the Bitcoin Order Protocol revolves around creating and managing orders with HTLC escrow:

### Creating orders

Create a new order with time lock and secret hash:
```bash
dfx canister call basic_bitcoin create_order '("03a1b2c3d4e5f6...", 1700000000, "abc123...")'
```

Parameters:
- `initiator_pubkey`: Public key of the order initiator
- `time_lock`: Unix timestamp for order expiration
- `secret_hash`: Hash of the secret for HTLC verification

### Viewing orders

Get details of a specific order:
```bash
dfx canister call basic_bitcoin get_order '(1)'
```

View all orders:
```bash
dfx canister call basic_bitcoin get_all_orders '()'
```

Get the next available order number:
```bash
dfx canister call basic_bitcoin get_next_order_no '()'
```

Get the Bitcoin address for an order:
```bash
dfx canister call basic_bitcoin get_order_address '(1)'
```

### Executing order withdrawals

Preview an order withdrawal before execution:
```bash
dfx canister call basic_bitcoin preview_order_withdrawal '(1, "03a1b2c3d4e5f6...")'
```

Execute withdrawal to HTLC:
```bash
dfx canister call basic_bitcoin execute_order_withdraw_to_htlc '(1, "03a1b2c3d4e5f6...", 100000)'
```

Parameters:
- `order_no`: The order number to withdraw from
- `responder_pubkey`: Public key of the responding party
- `amount_in_satoshi`: Amount to withdraw in satoshis
## Security Considerations

This Bitcoin Order Protocol implementation includes several important security features and considerations:

### Security Features

- **Hash Time-Locked Contracts (HTLC)**: Ensures atomic swaps without trust
- **Time-lock Protection**: Automatic refund mechanisms prevent fund loss
- **Cryptographic Verification**: Secret hash verification ensures secure settlement
- **Multi-signature Support**: Enhanced transaction security through multiple signatures

### Best Practices Implemented

- **Derivation Paths**: Keys are derived using structured derivation paths according to BIP-32
- **Key Caching**: Optimization to avoid repeated calls to signing APIs
- **Manual Transaction Construction**: Full control over transaction assembly and fee estimation
- **Cost Optimization**: Efficient use of threshold signing APIs

### Security Considerations

This project is built for hackathon purposes and should be thoroughly audited before production use. Important security considerations include:

- [Certify query responses](https://internetcomputer.org/docs/building-apps/security/data-integrity-and-authenticity#using-certified-variables-for-secure-queries) for balance and order queries
- [Use decentralized governance](https://internetcomputer.org/docs/building-apps/security/decentralization) like SNS for production deployments
- Implement proper access controls for order management functions
- Validate all cryptographic parameters and time locks
- Ensure proper handling of Bitcoin network confirmations

For more information on security best practices, refer to the [Internet Computer Security Guidelines](https://internetcomputer.org/docs/current/references/security/).

---

**Built for 1inch Hackathon by the Catalog Team**  
*Project: Bitcoin Order Protocol on Internet Computer*  
*Last updated: August 2025*
