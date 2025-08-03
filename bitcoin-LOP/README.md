# Bitcoin Order Protocol

A comprehensive Bitcoin smart contract on the Internet Computer that implements a decentralized order protocol with HTLC (Hash Time-Locked Contract) functionality. This project enables secure peer-to-peer Bitcoin transactions with time-locked escrow mechanisms and supports multiple Bitcoin address types.

---

## Table of Contents

* [Architecture](#architecture)
* [Features](#features)
* [Order Protocol](#order-protocol)
* [Mainnet Swap Flow](#mainnet-swap-flow)
* [Building and Deploying the Smart Contract Locally](#building-and-deploying-the-smart-contract-locally)

  * [1. Prerequisites](#1-prerequisites)
  * [2. Clone the Repository](#2-clone-the-repository)
  * [3. Start the ICP Execution Environment](#3-start-the-icp-execution-environment)
  * [4. Start Bitcoin Regtest](#4-start-bitcoin-regtest)
  * [5. Deploy the Smart Contract](#5-deploy-the-smart-contract)
* [Basic Bitcoin Operations](#basic-bitcoin-operations)

  * [Checking Balance](#checking-balance)
  * [Getting UTXOs](#getting-utxos)
  * [Fee Estimation](#fee-estimation)
* [Order Protocol Usage](#order-protocol-usage)

  * [Creating Orders](#creating-orders)
  * [Viewing Orders](#viewing-orders)
  * [Executing Order Withdrawals](#executing-order-withdrawals)
* [Security Considerations](#security-considerations)
* [Inscribe an Ordinal](#inscribe-an-ordinal)
* [Etch a Rune](#etch-a-rune)
* [Deploy a BRC-20 Token](#deploy-a-brc-20-token)
* [Notes on Implementation](#notes-on-implementation)

---

## Architecture

The Bitcoin Order Protocol integrates with the Internet Computer's built-in APIs to provide a secure and decentralized order management system:

* [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key) - For Bitcoin signature generation
* [Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr) - For Taproot transactions
* [Bitcoin API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md) - For Bitcoin network integration

For background on the ICP<>BTC integration, refer to the [Learn Hub](https://learn.internetcomputer.org/hc/en-us/articles/34211154520084-Bitcoin-Integration).

---

## Features

### 🚀 Core Functionality

* **Order Protocol**: Create and manage Bitcoin orders with HTLC escrow
* **Multi-address Support**: Compatible with P2PKH, P2WPKH, and P2TR address types
* **Time-locked Contracts**: Secure transactions with configurable time locks
* **Fee Estimation**: Dynamic Bitcoin network fee calculation
* **UTXO Management**: Efficient unspent transaction output handling

### 🔒 Security Features

* **Hash Time-Locked Contracts (HTLC)**: Trustless order execution
* **Secret Hash Verification**: Cryptographic proof of payment
* **Time-lock Protection**: Automatic refund mechanisms
* **Multi-signature Support**: Enhanced transaction security

---

## Order Protocol

The Bitcoin Order Protocol enables secure peer-to-peer transactions through a sophisticated escrow system:

1. **Order Creation**: Initiator creates an order with time lock and secret hash
2. **Order Address Generation**: Unique Bitcoin address generated for each order
3. **Funding**: Initiator funds the order address with Bitcoin
4. **Response**: Responder can preview and execute withdrawal to HTLC
5. **Settlement**: Cryptographic proof enables final settlement or time-lock refund

---

## Mainnet Swap Flow

Regular Bitcoin HTLCs have been used in Bitcoin for a long time.

The main restrictions on the Bitcoin side for integration with 1inch are as follows:

* The resolver addresses (taker) must be known before the maker commits funds.
* Partial fills are not possible.
* Auctions cannot be conducted.

To overcome the above challenges and extend 1inch to Bitcoin in a permissionless manner, we leverage **ICP Chain Fusion** technology.
Learn more about ICP Chain Fusion and why it is safe and permissionless:
[https://internetcomputer.org/docs/building-apps/chain-fusion/overview](https://internetcomputer.org/docs/building-apps/chain-fusion/overview)

### Swap Flow: BTC → Any Chain

This flow demonstrates execution steps on the Bitcoin side. The corresponding chain is assumed to have a ready HTLC:

1. **Maker** creates an order:

   ```
   create_order(maker_pubkey, timelock, secret_hash) -> order_no
   ```

2. Fetch the order's Bitcoin address:

   ```
   get_order_address(order_no) -> bitcoin_address
   ```

3. Maker funds the address with BTC.
4. **Taker** previews the withdrawal:

   ```
   preview_order_withdrawal(order_no, taker_pubkey) -> ..., htlc_address
   ```

5. If satisfied, taker proceeds to withdraw:

   ```
   execute_order_withdraw_to_htlc(order_no, taker_pubkey, amount)
   ```

6. Once funds reach the on-chain HTLC, **maker reveals the secret** and taker completes redemption.

---

## Building and Deploying the Smart Contract Locally

### 1. Prerequisites

* [x] [Rust toolchain](https://www.rust-lang.org/tools/install)
* [x] [Internet Computer SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
* [x] [Local Bitcoin testnet (regtest)](https://internetcomputer.org/docs/build-on-btc/btc-dev-env#create-a-local-bitcoin-testnet-regtest-with-bitcoind)
* [x] On macOS: install [Homebrew LLVM](https://formulae.brew.sh/formula/llvm) via `brew install llvm` for `wasm32-unknown-unknown` target support.

### 2. Start the ICP Execution Environment

```bash
dfx start --enable-bitcoin --bitcoin-node 127.0.0.1:18444
```

### 3. Start Bitcoin Regtest

```bash
bitcoind -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/bitcoin_data --port=18444
```

### 4. Deploy the Smart Contract

```bash
dfx deploy basic_bitcoin --argument '(variant { regtest })'
```

---

## Basic Bitcoin Operations

### Checking Balance

```bash
dfx canister call basic_bitcoin get_balance '("<bitcoin_address>")'
```

### Getting UTXOs

```bash
dfx canister call basic_bitcoin get_utxos '("<bitcoin_address>")'
```

### Fee Estimation

```bash
dfx canister call basic_bitcoin get_current_fee_percentiles '()'
```

---

## Order Protocol Usage

### Creating Orders

```bash
dfx canister call basic_bitcoin create_order '("03a1b2c3d4e5f6...", 1700000000, "abc123...")'
```

### Viewing Orders

```bash
dfx canister call basic_bitcoin get_order '(1)'
dfx canister call basic_bitcoin get_all_orders '()'
dfx canister call basic_bitcoin get_next_order_no '()'
dfx canister call basic_bitcoin get_order_address '(1)'
```

### Executing Order Withdrawals

Dry run preview:

```bash
dfx canister call basic_bitcoin preview_order_withdrawal '(1, "03a1b2c3d4e5f6...")'
```

Execute withdrawal:

```bash
dfx canister call basic_bitcoin execute_order_withdraw_to_htlc '(1, "03a1b2c3d4e5f6...", 100000)'
```

---

## Security Considerations

### Built-in Features

* HTLC-based settlement
* Time-lock enforced refunds
* Secret hash cryptographic validation
* Multisig support for enhanced security

### Best Practices Implemented

* Structured BIP-32 derivation paths
* Key caching to minimize API load
* Manual transaction construction with optimized fee estimation
* Efficient use of threshold signing APIs

### Additional Considerations

> This project is a hackathon prototype and **not audited for production**.

* Use [certified query responses](https://internetcomputer.org/docs/building-apps/security/data-integrity-and-authenticity#using-certified-variables-for-secure-queries)
* Adopt [decentralized governance](https://internetcomputer.org/docs/building-apps/security/decentralization)
* Implement strict access controls
* Validate all cryptographic/time-lock inputs rigorously
* Wait for sufficient Bitcoin confirmations before processing orders

For more, refer to [Internet Computer Security Guidelines](https://internetcomputer.org/docs/current/references/security/).

---

**Built for 1inch Hackathon by the Catalog Team**
*Project: Bitcoin Order Protocol on Internet Computer*
*Last updated: August 2025*
