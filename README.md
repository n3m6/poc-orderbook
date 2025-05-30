# Solana Onchain Orderbook Proof-of-Concept

<img src="./docs/hacker-trader.jpg" alt="drawing" width="400"/>

## Overview

This proof-of-concept demonstrates a high-performance orderbook implementation on Solana using a **sparse directory structure** with **price-level bucketing** - an architecture specifically designed to overcome Solana's account size limitations while maintaining real-time performance for financial applications.

The system achieves:
- **O(log d)** order placement/cancellation (d = directory size)
- **O(1)** best bid/ask access
- **O(log d + k)** range queries (k = results)
- Efficient sharding across Solana's 10MiB account limit
- Predictable compute unit consumption

## Core Architecture

### 1. Tiered Storage Structure

```
Orderbook State (1 account)
│
├── Directory Accounts (linked list)
│    │
│    └── Bucket Accounts (price ranges)
│         │
│         └── Order Pages (linked lists)
│              │
│              └── Individual Orders
```

### 2. Key Components

| Component          | Description                                                                 | Size Estimate |
|--------------------|-----------------------------------------------------------------------------|--------------|
| **OrderbookState** | Global state (symbol, tick size, best bid/ask)                              | ~200 bytes   |
| **Directory**      | Maps price ranges to buckets (sorted array for binary search)               | ~100 KB      |
| **PriceBucket**    | Manages 1000 price levels (stores aggregates, not individual orders)        | ~32 KB       |
| **OrderPage**      | Stores up to 64 orders at specific price level (linked list structure)      | ~4 KB        |

### 3. Capacity Scaling

| Structure       | Units per Account | Total Capacity          |
|-----------------|-------------------|-------------------------|
| Directory       | 2000 buckets      | 2000 price ranges       |
| Price Bucket    | 1000 price levels | 2,000,000 price levels  |
| Order Page      | 64 orders         | Virtually unlimited     |

## Key Features

1. **Price-Based Sharding**
    - Orders dynamically grouped into price range buckets
    - Active trading ranges use smaller buckets for granular updates
    - Sparse directory avoids allocating empty price ranges

2. **Efficient Operations**
   ```mermaid
   graph LR
   A[Place Order] --> B{Find Bucket}
   B --> C[Binary Search Directory]
   C --> D[Locate Price Level]
   D --> E{Order Page Available?}
   E -->|Yes| F[Add to Existing Page]
   E -->|No| G[Create New Order Page]
   ```

3. **Optimized Memory Usage**
    - Zero-copy deserialization for high-frequency access
    - Linked lists minimize data shifting
    - Aggregated price level data (total quantity) avoids full scans

4. **Hot/Cold Data Separation**
    - Dedicated bucket types for active vs. inactive price ranges
    - Smaller buckets near spread for tighter price granularity
    - Larger buckets for less active price areas

## Performance Characteristics

| Operation        | Compute Units | Accounts Touched | Complexity        |
|------------------|---------------|------------------|-------------------|
| Place Order      | 5,000-7,000   | 4-5              | O(log d + k)      |
| Cancel Order     | 3,000-5,000   | 3-4              | O(log d + k)      |
| Get Best Bid/Ask | 500-800       | 1                | O(1)             |
| Range Query      | 1,000/level   | 1 + n/levels     | O(log d + k)      |

## Getting Started

### Prerequisites
- Rust rustc 1.87.0+
- Solana CLI 2.1.22+
- Anchor 0.31.1+

### Installation
```bash
git clone https://github.com/n3m6/poc-orderbook.git
cd poc-orderbook
anchor build
anchor test
```

### Basic Workflow

1. **Initialize Orderbook**
```typescript
await program.methods.initializeOrderbook(
  baseMint, 
  quoteMint,
  new BN(100), // tick size
  new BN(10000) // bucket size
).accounts({ ... }).rpc();
```

2. **Place Bid Order**
```typescript
await program.methods.placeOrder(
  new BN(10500), // price
  new BN(500),   // quantity
  true           // isBid
).accounts({
  orderbook: orderbookState,
  directory: mainDirectory,
  bucket: targetBucket,
  orderPage: currentPage,
  // ... other accounts
}).rpc();
```

3. **Cancel Order**
```typescript
await program.methods.cancelOrder(
  orderId        // 128-bit order ID
).accounts({
  orderbook: orderbookState,
  bucket: targetBucket,
  orderPage: pageAccount,
  // ... other accounts
}).rpc();
```

4. **Query Best Bid**
```typescript
const state = await program.account.orderbookState.fetch(orderbookState);
console.log("Best bid:", state.bestBid.toString());
```

## Design Tradeoffs

1. **Pros**
    - Predictable performance under load
    - Natural fit for orderbook microstructure
    - Efficient memory utilization
    - Horizontal scaling via sharding
    - Optimized for common exchange workflows

2. **Cons**
    - Increased complexity for cross-bucket operations
    - Directory management overhead
    - Requires careful tick size configuration
    - More accounts to manage

## Next Steps for Production

1. Implement dynamic bucket resizing based on market volatility
2. Add order matching engine logic
3. Develop garbage collection for empty pages
4. Create snapshotting mechanism for market data feeds
5. Integrate with on-chain oracle for price validation

## Why This Architecture Wins for Solana

Traditional approaches (binary trees, arrays) fail on Solana due to:
- **Account Size Limits**: 10MiB restricts single-account data structures
- **Compute Limits**: Tree rotations become prohibitively expensive
- **Write Contention**: Global orderbooks create account conflicts

This sparse directory/bucketing approach solves these by:
- Sharding naturally by price ranges
- Minimizing per-operation account touches
- Eliminating expensive rebalancing operations
- Leveraging Solana's strength in account parallelism

## Contributing

This POC welcomes contributions! Please see our [Contribution Guidelines](CONTRIBUTING.md) for details.

## License

GPL-3.0 See [LICENSE](LICENSE) for details.