# Cross-Chain Order Resolution System

## Overview

This project implements a comprehensive cross-chain order resolution system with multiple microservices working together to facilitate secure cross-chain transactions.

## 🏗️ Architecture

### Services Developed

1. **Resolver Contract** - Smart contract handling order resolution logic
2. **Relayer Service** - API service for order submission and management
3. **Resolver Service** - Service for filling and executing orders
4. **Watcher Service** - Blockchain event monitoring and state management
5. **Client** - Frontend application for user interaction

## 📋 Order Status Flow

```
PENDING → SRC_FILLED → DEST_FILLED → SRC_SETTLED → DST_SETTLED → FULFILLED
```

## 🔧 Technology Stack

- **Smart Contracts**: Solidity with Foundry
- **Backend Services**: Rust (Relayer, Resolver, Watcher)
- **Frontend**: TypeScript/JavaScript
- **Blockchain**: Multi-chain support (Ethereum, Base, Monad, etc.)

## 🔄 Cross-Chain Order Flow

The system follows a sophisticated multi-step process to ensure secure cross-chain transactions:

### Step 1: Order Submission

The user submits an **Order Intent** to our relayer `submit` endpoint.

### Step 2: Order Registration

The **Relayer** updates our database with all the Order details, which is now available for any `Resolver` to fill.

### Step 3: Order Resolution Initiation

**Resolver service** continuously polls the Relayer and starts filling the order.

### Step 4: Source Escrow Deployment

The Resolver first deploys **source escrow contract**, where LOP pulls the funds of Maker into the Source Escrow contract.

### Step 5: Source Chain Monitoring

In the background, our **watcher service** keeps listening to Source Chain Escrow Factory contract's `SrcEscrowCreated` event and updates the `SrcChainImmutables` to the DB.

### Step 6: Destination Immutables Construction

Our designated **Watcher service** constructs the `DestChainImmutables` and then updates the DB, changing the status of the Order to `SRC_FILLED`.

### Step 7: Destination Escrow Deployment

The resolver service checks this status and then picks the `DestChainImmutables` and deploys the **DestEscrow contract** on the Dest Chain.

### Step 8: Destination Chain Monitoring

Again the **Watcher service** listens to this on-chain Event and updates the status of the Order to `DEST_FILLED`, also updating the `src_withdraw_immutables` and `dest_chain_immutables`.

### Step 9: Source Chain Withdrawal

The resolver service checks this order status and then executes the **Src chain withdrawal**, from which the Taker receives the intended funds on the Source chain.

### Step 10: Source Settlement Confirmation

The watcher updates the STATUS to `SRC_SETTLED`.

### Step 11: Destination Withdrawal

The Resolver finally withdraws from the **dest escrow**, which results in the **MAKER** receiving their funds on the Dest chain.

### Step 12: Order Completion

Finally, the order status is set to `FULFILLED` by our watcher service, marking the completion of our complete cross-chain order.

## 🎯 Key Features

- **Secure Cross-Chain Transactions**: Multi-step verification and escrow-based security
- **Real-Time Monitoring**: Continuous blockchain event monitoring
- **Automated Resolution**: Seamless order filling and execution
- **State Management**: Comprehensive order status tracking
- **Multi-Chain Support**: Flexible architecture supporting multiple blockchain networks
