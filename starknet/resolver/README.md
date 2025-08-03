# Starknet Resolver

A TypeScript resolver that fetches active orders from an API and processes Starknet orders by calling the `create_source` function on the resolver contract.

## Features

- Fetches active orders from `http://localhost:4455/orders/active`
- Filters orders by Starknet chain ID
- Processes orders by calling the resolver contract's `create_source` function
- Continuous polling with configurable intervals
- Error handling and graceful shutdown

## Installation

```bash
cd starknet/resolver
npm install
```

## Configuration

Edit `src/config.ts` to configure:

- Resolver private key and address
- Resolver contract address
- API endpoint
- Starknet RPC URL
- Polling interval

## Usage

### Development

```bash
npm run dev
```

### Production

```bash
npm run build
npm start
```

### Watch mode (for development)

```bash
npm run watch
```

## How it works

1. The resolver fetches all active orders from the API endpoint
2. Filters orders where `srcChainId` matches Starknet's chain ID
3. For each Starknet order:
   - Converts the order to the required format
   - Parses the signature
   - Generates a new order hash for the escrow
   - Calls the `create_source` function on the resolver contract
4. Continues polling at the configured interval

## API Response Format

The resolver expects the API to return orders in this format:

```json
{
  "status": "ok",
  "result": {
    "meta": {
      "totalItems": 150,
      "itemsPerPage": 100,
      "totalPages": 2,
      "currentPage": 1
    },
    "items": [
      {
        "orderHash": "0x...",
        "signature": "0x...",
        "deadline": 1640995200000,
        "auctionStartDate": "2024-01-01T00:00:00Z",
        "auctionEndDate": "2024-01-02T00:00:00Z",
        "remainingMakerAmount": "1000000000000000000",
        "extension": "0x...",
        "srcChainId": 1,
        "dstChainId": 137,
        "order": {
          "salt": "0x...",
          "makerAsset": "0x...",
          "takerAsset": "0x...",
          "maker": "0x...",
          "receiver": "0x...",
          "makingAmount": "1000000000000000000",
          "takingAmount": "2000000000000000000",
          "makerTraits": "0"
        },
        "orderType": "multiple_fills",
        "secrets": [
          {
            "index": 0,
            "secret": null,
            "secretHash": "0x..."
          }
        ]
      }
    ]
  }
}
```

## Error Handling

The resolver includes comprehensive error handling:

- API connection errors
- Starknet transaction failures
- Invalid order data
- Network timeouts

All errors are logged and the resolver continues processing other orders.

## Logging

The resolver provides detailed logging:

- Initialization status
- Number of orders found and processed
- Transaction hashes for successful operations
- Error details for failed operations

## Stopping the Resolver

The resolver can be stopped gracefully by sending SIGINT (Ctrl+C) or SIGTERM signals. 