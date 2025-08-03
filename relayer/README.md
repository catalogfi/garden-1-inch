# Garden 1-Inch Relayer

Garden 1-Inch Relayer serves as a centralized orderbook that enables users to submit, track, and manage cross-chain swaps.

## Response Format

All API responses follow a consistent format:

```json
{
  "status": "ok" | "error",
  "result": <response_data> | null,
  "error": <error_message> | null
}
```

- `status`: Indicates if the request was successful (`"ok"`) or encountered an error (`"error"`)
- `result`: Contains the actual response data when successful
- `error`: Contains error details when the request fails

## Endpoints

### 1. Health Check

**GET** `/health`

Check if the API is running.

**Response:**
```
Online
```

**Example:**
```bash
curl http://localhost:4455/health
```

---

### 2. Submit Order

**POST** `/relayer/submit`

Submit a cross-chain order that resolvers will be able to fill.

**Request Body:**
```json
{
  "order": {
    "salt": "string",
    "makerAsset": "0x...",
    "takerAsset": "0x...",
    "maker": "0x...",
    "receiver": "0x...",
    "makingAmount": "1000000000000000000",
    "takingAmount": "2000000000000000000",
    "makerTraits": "0"
  },
  "srcChainId": 1,
  "dstChainId": 137,
  "signature": "0x...",
  "extension": "0x...",
  "quoteId": "quote_123",
  "orderType": "single_fill" | "multiple_fills",
  "secrets": [
    {
      "index": 0,
      "secret": null,
      "secretHash": "0x..."
    }
  ],
  "deadline": 1640995200000
}
```

**Field Descriptions:**
- `order.salt`: Unique identifier for the order
- `order.makerAsset`: Source chain address of the maker asset (Ethereum address)
- `order.takerAsset`: Destination chain address of the taker asset (Ethereum address)
- `order.maker`: Source chain address of the maker (wallet or contract address)
- `order.receiver`: Destination chain address of the wallet or contract who will receive filled amount
- `order.makingAmount`: Order maker's token amount (BigDecimal string)
- `order.takingAmount`: Order taker's token amount (BigDecimal string)
- `order.makerTraits`: Includes flags like allow multiple fills, partial fill allowed, price improvement, nonce, deadline (default: "0")
- `srcChainId`: Source chain ID
- `dstChainId`: Destination chain ID
- `signature`: Signature of the cross-chain order typed data (using signTypedData v4)
- `extension`: Interaction call data (ABI encoded)
- `quoteId`: Quote ID of the quote with presets
- `orderType`: Order type - "single_fill" or "multiple_fills"
- `secrets`: Secret entries containing index, secret, and secret_hash (required for multiple_fills orders)
- `deadline`: Deadline by which the order must be filled (Unix timestamp in milliseconds)

**Response:**
- **Status Code:** `202 Accepted` (success)
- **Status Code:** `400 Bad Request` (validation error)
- **Status Code:** `500 Internal Server Error` (server error)

**Example:**
```bash
curl -X POST http://localhost:4455/relayer/submit \
  -H "Content-Type: application/json" \
  -d '{
    "order": {
      "salt": "0x1234567890abcdef",
      "makerAsset": "0x1234567890123456789012345678901234567890",
      "takerAsset": "0x0987654321098765432109876543210987654321",
      "maker": "0x1111111111111111111111111111111111111111",
      "receiver": "0x2222222222222222222222222222222222222222",
      "makingAmount": "1000000000000000000",
      "takingAmount": "2000000000000000000",
      "makerTraits": "0"
    },
    "srcChainId": 1,
    "dstChainId": 137,
    "signature": "0x1234567890abcdef...",
    "extension": "0x1234567890abcdef",
    "quoteId": "quote_123",
    "orderType": "multiple_fills",
    "secrets": [
      {
        "index": 0,
        "secret": null,
        "secretHash": "0xabcdef1234567890..."
      }
    ],
    "deadline": 1640995200000
  }'
```

---

### 3. Get Active Orders

**GET** `/orders/active`

Retrieve all unmatched orders with pagination.

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Number of items per page (default: 100, max: 500)

**Response:**
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

**Example:**
```bash
curl "http://localhost:4455/orders/active?page=1&limit=10"
```

---

### 4. Get Order by Hash

**GET** `/orders/{order_hash}`

Retrieve a specific order by its hash.

**Path Parameters:**
- `order_hash`: The order hash to retrieve

**Response:**
```json
{
  "status": "ok",
  "result": {
    "createdAt": "2024-01-01T00:00:00Z",
    "updatedAt": "2024-01-01T00:00:00Z",
    "orderHash": "0x...",
    "srcChainId": 1,
    "dstChainId": 137,
    "maker": "0x...",
    "receiver": "0x...",
    "makerAsset": "0x...",
    "takerAsset": "0x...",
    "makingAmount": "1000000000000000000",
    "takingAmount": "2000000000000000000",
    "salt": "0x...",
    "makerTraits": "0",
    "signature": "0x...",
    "extension": "0x...",
    "orderType": "multiple_fills",
    "secrets": [...],
    "status": "unmatched",
    "deadline": 1640995200000,
    "auctionStartDate": "2024-01-01T00:00:00Z",
    "auctionEndDate": "2024-01-02T00:00:00Z",
    "srcEscrowAddress": "0x...",
    "dstEscrowAddress": "0x...",
    "srcTxHash": "0x...",
    "dstTxHash": "0x...",
    "filledMakerAmount": "0",
    "filledTakerAmount": "0"
  }
}
```

**Example:**
```bash
curl http://localhost:4455/orders/0x1234567890abcdef...
```

---

### 5. Get Orders by Chain

**GET** `/orders/chain/{chain_id}`

Retrieve all orders for a specific chain.

**Path Parameters:**
- `chain_id`: The chain ID to filter orders by

**Response:**
```json
{
  "status": "ok",
  "result": [
    {
      "createdAt": "2024-01-01T00:00:00Z",
      "updatedAt": "2024-01-01T00:00:00Z",
      "orderHash": "0x...",
      "srcChainId": 1,
      "dstChainId": 137,
      // ... other order fields
    }
  ]
}
```

**Example:**
```bash
curl http://localhost:4455/orders/chain/1
```

---

### 6. Submit Secret

**POST** `/relayer/secret`

Submit a secret for an order fill. This endpoint is used when an order reaches the `finality_confirmed` status.

**Request Body:**
```json
{
  "secret": "abcdef1234567890...",
  "orderHash": "0x..."
}
```

**Field Descriptions:**
- `secret`: A secret for the fill hashlock (hex string without 0x prefix)
- `orderHash`: Order hash

**Response:**
- **Status Code:** `202 Accepted` (success)
- **Status Code:** `400 Bad Request` (validation error)
- **Status Code:** `404 Not Found` (order not found)
- **Status Code:** `500 Internal Server Error` (server error)

**Example:**
```bash
curl -X POST http://localhost:4455/relayer/secret \
  -H "Content-Type: application/json" \
  -d '{
    "secret": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    "orderHash": "0x1234567890abcdef..."
  }'
```

---

### 7. Get Secret

**GET** `/orders/secret/{order_hash}`

Retrieve secrets for a specific order.

**Path Parameters:**
- `order_hash`: The order hash to retrieve secrets for

**Response:**
```json
{
  "status": "ok",
  "result": {
    "secret": "abcdef1234567890...",
    "orderHash": "0x..."
  }
}
```

**Example:**
```bash
curl http://localhost:4455/orders/secret/0x1234567890abcdef...
```

---

## Order Statuses

The API supports the following order statuses:

- `unmatched`: Order is waiting to be filled
- `source_filled`: Order has been filled on the source chain
- `destination_filled`: Order has been filled on the destination chain
- `finality_confirmed`: Finality has been confirmed (secrets can be submitted)
- `source_withdraw_pending`: Source withdrawal is pending
- `destination_withdraw_pending`: Destination withdrawal is pending
- `source_settled`: Order has been settled on the source chain
- `destination_settled`: Order has been settled on the destination chain
- `source_refunded`: Order has been refunded on the source chain
- `destination_refunded`: Order has been refunded on the destination chain
- `source_canceled`: Order has been canceled on the source chain
- `destination_canceled`: Order has been canceled on the destination chain
- `expired`: Order has expired

## Order Types

- `single_fill`: Order can only be filled by one resolver
- `multiple_fills`: Order can be filled by multiple resolvers


### Complete Order Flow

1. **Submit an order:**
```bash
curl -X POST http://localhost:4455/relayer/submit \
  -H "Content-Type: application/json" \
  -d '{
    "order": {
      "salt": "0x1234567890abcdef",
      "makerAsset": "0x1234567890123456789012345678901234567890",
      "takerAsset": "0x0987654321098765432109876543210987654321",
      "maker": "0x1111111111111111111111111111111111111111",
      "receiver": "0x2222222222222222222222222222222222222222",
      "makingAmount": "1000000000000000000",
      "takingAmount": "2000000000000000000",
      "makerTraits": "0"
    },
    "srcChainId": 1,
    "dstChainId": 137,
    "signature": "0x1234567890abcdef...",
    "extension": "0x1234567890abcdef",
    "quoteId": "quote_123",
    "orderType": "multiple_fills",
    "secrets": [
      {
        "index": 0,
        "secret": null,
        "secretHash": "0xabcdef1234567890..."
      }
    ],
    "deadline": 1640995200000
  }'
```

2. **Get active orders:**
```bash
curl "http://localhost:4455/orders/active?page=1&limit=10"
```

3. **Get specific order:**
```bash
curl http://localhost:4455/orders/0x1234567890abcdef...
```

4. **Submit secret (when order reaches finality_confirmed):**
```bash
curl -X POST http://localhost:4455/relayer/secret \
  -H "Content-Type: application/json" \
  -d '{
    "secret": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    "orderHash": "0x1234567890abcdef..."
  }'
```

5. **Get secrets:**
```bash
curl http://localhost:4455/orders/secret/0x1234567890abcdef...
``` 