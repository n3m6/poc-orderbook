# Solana Onchain Orderbook Proof-of-Concept

<img src="./docs/hacker-trader.jpg" alt="drawing" width="830"/>

## Overview

This proof-of-concept demonstrates a high-performance orderbook implementation on Solana using a **sparse directory 
structure** with **price-level bucketing** - an architecture specifically designed to overcome Solana's account size 
limitations while maintaining real-time performance for financial applications. For a simpler explanation of the 
data structure, read the [Data Structure Overview](docs/data-structure.md).

## Core Architecture

### Tiered Storage Structure

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
    - Linked lists minimize data shifting

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