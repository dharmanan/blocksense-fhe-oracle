# blocksense-fhe-oracle

Blocksense + FHE (Fully Homomorphic Encryption, powered by Zama/Concrete) proof-of-concept for privacy-preserving prediction market oracle.

## 🎯 Objective

Build a decentralized oracle system that aggregates provider predictions using homomorphic encryption, settles on-chain with threshold decryption, and resolves disputes via jury mechanism.

## 📋 Project Structure

```
blocksense-fhe-oracle/
├── contracts/
│   └── FHEOracle.sol
├── docs/
│   ├── INTEGRATION.md
│   └── mvp-sprint-plan.md
├── examples/
│   ├── Cargo.toml
│   └── zama_integer_sum.rs
├── sdk/
│   ├── package.json
│   ├── .eslintrc.json
│   └── encrypt.js
├── .github/workflows/
│   └── ci.yaml
├── .gitignore
├── LICENSE
└── README.md
```

## 🚀 Quick Start

```bash
git clone https://github.com/dharmanan/blocksense-fhe-oracle.git
cd blocksense-fhe-oracle
git checkout dev

# Rust examples
cd examples && cargo build && cargo test

# Node SDK
cd ../sdk && npm install && npm run lint
```

## 🏗️ Architecture

Data Provider → Quantize & Encrypt → FHE Compute → Threshold Decrypt → On-Chain Settlement

### Key Components

- **`contracts/FHEOracle.sol`**: Solidity oracle contract for on-chain settlement
- **`examples/zama_integer_sum.rs`**: Rust POC for homomorphic aggregation
- **`sdk/encrypt.js`**: Node.js encryption adapter for data providers
- **`docs/INTEGRATION.md`**: Technical architecture and API reference
- **`docs/mvp-sprint-plan.md`**: 10-week development roadmap

## 📖 Documentation

- **INTEGRATION.md**: Technical specification for FHE pipeline
- **mvp-sprint-plan.md**: Week-by-week development plan

## 🧪 Testing

```bash
# Rust tests
cd examples && cargo test --verbose

# Node linting
cd ../sdk && npm run lint
```

## 📝 License

MIT — See LICENSE for details.

**Status**: 🚧 POC Phase (Week 1-2)
