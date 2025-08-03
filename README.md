# ⚡ Cross-Chain Order Resolution System

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

* **CREATE2 Magic**: All LOP (Limit Order Protocol) contracts are deployed using `CREATE2`, resulting in **deterministic addresses across chains**. This eliminates the need for registries, config mappings, or hardcoded addresses—making the developer and integrator experience significantly smoother.

---

## 🔄 Order Lifecycle (High-Level)

```
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

* Order status → `DEST_FILLED`
* `src_withdraw_immutables` and `dest_chain_immutables`

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

| Layer           | Tech                              |
| --------------- | --------------------------------- |
| Smart Contracts | Solidity + Foundry                |
| Backend         | Rust (Relayer, Resolver, Watcher) |
| Client Scripts  | TypeScript / JavaScript           |
| Blockchain      | Ethereum, Base, Monad (modular)   |

---

## ✨ Key Features

* 🔐 **Secure Cross-Chain Transactions**: Escrow-based system ensures trustless execution
* 🤖 **Automated Resolution**: Resolvers handle order execution end-to-end
* 👁️ **Real-Time Monitoring**: Watchers listen to on-chain events and sync off-chain state
* 🧠 **Stateful Processing**: Fine-grained status tracking for each order
* 🌐 **Multi-Chain Support**: Built to support any EVM-compatible chain
* 🧩 **CREATE2 Deployments**: Contracts are deployed deterministically, simplifying integration across chains
* ⚙️ **Modular Microservices**: Each backend service can be deployed, scaled, and debugged independently

---

## 📎 Example Use Case

> Alice on Ethereum wants to swap 1 ETH for 1000 USDC from Bob on Base.
> Our system escrows both sides, verifies chain events, and resolves the trade atomically. Neither party needs to trust the other—just the system.
