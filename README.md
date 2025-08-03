# ⚡ Cross-Chain Order Resolution System

## 🚀 Overview

We built a full-stack cross-chain order resolution system that bridges multiple blockchains, enabling **secure**, **automated**, and **real-time** execution of trades across ecosystems. Powered by microservices and deterministic contracts, it streamlines the experience for both users and developers.

## 🧠 Architecture Breakdown

### 🔩 Core Components

1. **📝 Resolver Contract** – Handles order resolution logic on-chain
2. **🚚 Relayer Service** – Exposes APIs for submitting and managing orders
3. **⚙️ Resolver Service** – Executes matched orders and manages swaps
4. **👁️ Watcher Service** – Monitors blockchain events to trigger workflows
5. **🧪 Client** – Scriptable interface for interacting with the system (great for bots & integrations)

### 🧬 Deployment Strategy

* **CREATE2 Magic**: We deploy our custom Limit Order Protocol (LOP) contracts on all supported chains using `CREATE2`, giving them **the same address everywhere**. This makes integration dead simple and lets frontends and backends use one address per chain without lookups or configs.

## 🔄 Order Lifecycle

```
UNMATCHED → SRC_FILLED → DEST_FILLED → SRC_SETTLED → DST_SETTLED → FULFILLED
```

Each state is tracked and enforced across chains to guarantee atomic resolution and prevent race conditions.

## 🧰 Tech Stack

* **Smart Contracts**: Solidity + Foundry
* **Backends**: Rust microservices (Relayer, Resolver, Watcher)
* **Client**: TypeScript / JavaScript
* **Chains**: Ethereum, Base, Monad, and more (modular by design)

## ✨ Key Features

* 🔐 **Cross-Chain Security** – Escrow-based logic ensures trustless fulfillment
* 👁️ **Live Monitoring** – Watchers track events and trigger resolution logic instantly
* 🤖 **Automated Execution** – Orders get matched, filled, and settled with zero manual steps
* 🧠 **Stateful Logic** – Fine-grained order states tracked across services
* 🌉 **Multi-Chain Ready** – Easily plug in any EVM-compatible chain
* 🧩 **Predictable Contracts via CREATE2** – No need for registries or discovery—contracts deploy to the same address on every chain, streamlining DX.