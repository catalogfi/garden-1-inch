# ⚡ Cross-Chain Order Resolution System

## 🏆 Competing Fusion+ Tracks

Monad · Etherlink · Tezos · TRON · Starknet · ICP · Bitcoin

---

## 📖 Additional Resources

-   [Bitcoin Order Protocol](https://github.com/catalogfi/garden-1-inch/blob/main/bitcoin-LOP/README.md) – Implementation details for the Bitcoin HTLC flow.
-   [Relayer Docs](https://github.com/catalogfi/garden-1-inch/blob/main/relayer/README.md) – Overview of the relayer’s logic and responsibilities.

## 🛰️ Deployments

> All core contracts are deployed via `CREATE2`, ensuring deterministic addresses across chains—no config mappings or address lookups needed.

---

### 🔵 Monad Testnet

| Contract                 | Address                                                                                                                              |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| **Limit Order Protocol** | [`0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2`](https://testnet.monadexplorer.com/address/0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2) |
| **Escrow Factory**       | [`0xa62dF4c42fFd8a352436461f3A3542bF2EFb06bF`](https://testnet.monadexplorer.com/address/0xa62dF4c42fFd8a352436461f3A3542bF2EFb06bF) |
| **Resolver**             | [`0x2Ccb1d9b36c0dE06195169d34fD64427F735186b`](https://testnet.monadexplorer.com/address/0x2Ccb1d9b36c0dE06195169d34fD64427F735186b) |
| **True ERC20**           | [`0x19eAC199abcc6f8dDe59198fcA5d44513B519368`](https://testnet.monadexplorer.com/token/0x19eAC199abcc6f8dDe59198fcA5d44513B519368)   |

---

### 🟠 Tron

| Contract       | Address                                                                                                                      |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **HTLC**       | [`TN4s34sk7MAowHG99BtZ5ELPui1ubjcwok`](https://nile.tronscan.org/#/contract/TN4s34sk7MAowHG99BtZ5ELPui1ubjcwok/transactions) |
| **True ERC20** | [`TS8BG6McvyLia2U9DFf9JE2CCRRyYyxUQC`](https://nile.tronscan.org/#/token20/TS8BG6McvyLia2U9DFf9JE2CCRRyYyxUQC)               |

---

### 🟣 Etherlink

| Contract                 | Address                                                                                                                                            |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Limit Order Protocol** | [`0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2`](https://testnet.explorer.etherlink.com/address/0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2?tab=logs) |
| **Escrow Factory**       | [`0x30d24e9d1Fbffad6883E8632c5ad4216c9A86dFC`](https://testnet.explorer.etherlink.com/address/0x30d24e9d1Fbffad6883E8632c5ad4216c9A86dFC?tab=logs) |
| **Resolver**             | [`0x4dfaBf46CCDd6b36a275b0b22f5C2077120914C9`](https://testnet.explorer.etherlink.com/address/0x4dfaBf46CCDd6b36a275b0b22f5C2077120914C9?tab=txs)  |
| **True ERC20**           | [`0x19eAC199abcc6f8dDe59198fcA5d44513B519368`](https://testnet.explorer.etherlink.com/token/0x19eAC199abcc6f8dDe59198fcA5d44513B519368)            |

---

### 🟡 Base Sepolia

| Contract                 | Address                                                                                                                         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| **Limit Order Protocol** | [`0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2`](https://sepolia.basescan.org/address/0xf850CF9A70Fe8279F49739F1A14528D8BCe675e2) |
| **Escrow Factory**       | [`0x048975f98b998796d1cF54DE3A3Fc2bE01d891Fd`](https://sepolia.basescan.org/address/0x048975f98b998796d1cF54DE3A3Fc2bE01d891Fd) |
| **Resolver**             | [`0xfdeF9FF4A8677F5ab235b4F1c98426F591E560D5`](https://sepolia.basescan.org/address/0xfdeF9FF4A8677F5ab235b4F1c98426F591E560D5) |
| **True ERC20**           | [`0x19eAC199abcc6f8dDe59198fcA5d44513B519368`](https://sepolia.basescan.org/address/0x19eAC199abcc6f8dDe59198fcA5d44513B519368) |

---

### 🟥 Starknet Sepolia

| Contract           | Address                                                                                                                                                                                 |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Escrow Factory** | [`0x02b3021e22c36d1b709c819d4c08b5ffcfe745eaac6aa3e9c141e098b802287c`](https://sepolia.starkscan.co/contract/0x02b3021e22c36d1b709c819d4c08b5ffcfe745eaac6aa3e9c141e098b802287c#events) |
| **Resolver**       | [`0x06b96700855961261698513b949b53a5ee4162efcbbf7a6eb6a2382d89989433`](https://sepolia.starkscan.co/contract/0x06b96700855961261698513b949b53a5ee4162efcbbf7a6eb6a2382d89989433#events) |
| **True ERC20**     | [`0x02fb897ed33fbd7f3b68bb51b3a1f1e94255d71c327c4447ec4db462848752bd`](https://sepolia.starkscan.co/token/0x02fb897ed33fbd7f3b68bb51b3a1f1e94255d71c327c4447ec4db462848752bd#transfers) |

---

### 🟠 Bitcoin Order Protocol (ICP)

| Component         | Details                                                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------------------------ |
| **Canister URL**  | [`rqnxq-kiaaa-aaaao-qkewq-cai`](https://a4gq6-oaaaa-aaaab-qaa4q-cai.icp0.io/?id=rqnxq-kiaaa-aaaao-qkewq-cai) |
| **Hosting Layer** | Internet Computer (ICP) – Native Bitcoin integration, fully onchain order protocol                           |

> Deployed natively on **ICP**, this protocol executes trustless Bitcoin swaps directly on Bitcoin mainnet—no bridges, no wrapped assets. The ICP canister orchestrates the trade logic, using native BTC calls.

#### 📦 Live Bitcoin Mainnet Transactions

-   🟢 **Maker funds order protocol**
    [`cd7ba3...336a`](https://mempool.space/tx/cd7ba38f2793b796b5fd7cc8fc8c24833a8e3096e4d034240487092ed0e4336a)

-   🔄 **Taker pulls funds into HTLC**
    [`af9332...a7da`](https://mempool.space/tx/af93329917d6a6a4f0a0cdd0d15cadddde848acff242a910d7d349bb3879a7da)

-   ✅ **Taker redeems from HTLC**
    [`65731a...3120`](https://mempool.space/tx/65731a12b1e94c2bb6f68983150fa2fd2acb88646c2262986bd4867a17833120)

---

## Deploying from ICP Ninja

This project can be deployed directly to the Internet Computer using ICP Ninja, where it connects to Bitcoin **testnet4**.

> Note: Canisters deployed via ICP Ninja remain live for 50 minutes after signing in with your Internet Identity.

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/catalogfi/garden-1-inch/tree/main/bitcoin-LOP)

## 🚀 Overview

We built a full-stack **cross-chain order resolution system** that securely executes trades across multiple blockchains using microservices, event-based workflows, and deterministic smart contracts. This system allows seamless coordination between users on different chains using escrow contracts, automated resolvers, and real-time watchers.

Our architecture ensures atomic, trustless execution—backed by deterministic deployments and a stateful backend that tracks every order from intent to fulfillment.

---

## 🧠 Architecture Breakdown

### 🔩 Core Components

1. **📝 Resolver Contract** – Smart contract that encapsulates order validation and escrow logic
2. **🚚 Relayer Service** – Exposes APIs for submitting and managing user orders
3. **⚙️ Resolver Service** – Automatically matches and resolves orders between users across chains
4. **👁️ Watcher Service** – Listens to on-chain events and updates the off-chain state accordingly
5. **🧪 Client** – Integration scripts or minimal frontend to test end-to-end flows

### 🧬 Deployment Strategy

-   **CREATE2 Magic**: All LOP (Limit Order Protocol) contracts are deployed using `CREATE2`, resulting in **deterministic addresses across chains**. This eliminates the need for registries, config mappings, or hardcoded addresses—making the developer and integrator experience significantly smoother.

---

## 🔄 Order Lifecycle (High-Level)

```code
UNMATCHED → SRC_FILLED → DEST_FILLED → SRC_SETTLED → DST_SETTLED → FULFILLED
```

Each state is actively monitored and enforced by our system to ensure secure and atomic cross-chain fulfillment.

---

## 📋 Cross-Chain Order Flow (Detailed)

### Step 1: 📝 Order Submission

The user submits an **Order Intent** to the `Relayer Service` via a `/submit` API.

---

### Step 2: 🗃️ Order Registration

The `Relayer Service` saves order details to the database, making them visible to any available `Resolver` node.

---

### Step 3: 🤖 Resolution Initiation

The `Resolver Service` polls for new unmatched orders and begins execution.

---

### Step 4: 🔐 Source Escrow Deployment

The `Resolver` deploys a **Source Escrow Contract**. The LOP contract pulls the **maker's funds** into this escrow.

---

### Step 5: 👁️ Source Chain Monitoring

The `Watcher Service` listens for the `SrcEscrowCreated` event and updates the order status and immutable parameters (`src_chain_immutables`) in the database.

---

### Step 6: 🧬 Dest Chain Immutables Construction

Once the source is confirmed, the watcher builds the `DestChainImmutables` and sets the order status to `SRC_FILLED`.

---

### Step 7: 📦 Destination Escrow Deployment

Using the new status and immutables, the `Resolver` deploys the **Dest Escrow Contract** on the destination chain.

---

### Step 8: 👀 Destination Chain Monitoring

The `Watcher Service` tracks this event and updates:

-   Order status → `DEST_FILLED`
-   `src_withdraw_immutables` and `dest_chain_immutables`

---

### Step 9: 💸 Source Chain Withdrawal

The `Resolver` initiates **source escrow withdrawal**, allowing the **Taker** to receive funds on the **source chain**.

---

### Step 10: ✅ Source Settlement Confirmation

Watcher confirms the withdrawal and updates the status to `SRC_SETTLED`.

---

### Step 11: 💰 Destination Withdrawal

The `Resolver` then finalizes the **destination escrow withdrawal**, allowing the **Maker** to receive funds on the **dest chain**.

---

### Step 12: 🎉 Fulfillment

The `Watcher` confirms the final withdrawal and updates the order status to `FULFILLED`.

---

## 🧰 Tech Stack

| Layer           | Tech                                                                                |
| --------------- | ----------------------------------------------------------------------------------- |
| Smart Contracts | Solidity + Foundry                                                                  |
| Backend         | Rust (Relayer, Resolver, Watcher)                                                   |
| Client Scripts  | TypeScript / JavaScript                                                             |
| Blockchain      | Ethereum, Base, Monad (modular), Etherlink, TRON, Starknet, Bitcoin and ICP         |
| Relayer API     | [Docs here](https://github.com/catalogfi/garden-1-inch/blob/main/relayer/README.md) |

---

## ✨ Key Features

-   🔐 **Secure Cross-Chain Transactions**: Escrow-based system ensures trustless execution
-   🤖 **Automated Resolution**: Resolvers handle order execution end-to-end
-   👁️ **Real-Time Monitoring**: Watchers listen to on-chain events and sync off-chain state
-   🧠 **Stateful Processing**: Fine-grained status tracking for each order
-   🌐 **Multi-Chain Support**: Built to support any EVM-compatible chain
-   🧩 **CREATE2 Deployments**: Contracts are deployed deterministically, simplifying integration across chains
-   ⚙️ **Modular Microservices**: Each backend service can be deployed, scaled, and debugged independently

---

## 📎 Example Use Case

> Alice on Ethereum wants to swap 1 ETH for 1000 USDC from Bob on Base.
> Our system escrows both sides, verifies chain events, and resolves the trade atomically. Neither party needs to trust the other—just the system.
