# blocksense-fhe-oracle# blocksense-fhe-oracle# blocksense-fhe-oracle



Blocksense + FHE (Fully Homomorphic Encryption, powered by Zama/Concrete) proof-of-concept for privacy-preserving prediction market oracle.Blocksense + FHE (Fully Homomorphic Encryption, powered by Zama/Concrete) proof-of-concept for privacy-preserving prediction market oracle.



## 🎯 Objective## 🎯 Objective



Build a decentralized oracle system that:Build a decentralized oracle system that:



1. **Aggregates provider predictions** using homomorphic encryption (no decryption until finality)1. **Aggregates provider predictions** using homomorphic encryption (no decryption until finality)

2. **Computes results privately** on encrypted data (Zama FHE)2. **Computes results privately** on encrypted data (Zama FHE)

3. **Settles on-chain** with distributed threshold decryption (MPC)3. **Settles on-chain** with distributed threshold decryption (MPC)

4. **Resolves disputes** via jury/arbitration mechanism4. **Resolves disputes** via jury/arbitration mechanism

5. **Distributes payouts** according to market outcomes5. **Distributes payouts** according to market outcomes



## 📋 Project Structure## 📋 Project Structure



``````

blocksense-fhe-oracle/blocksense-fhe-oracle/

├── contracts/├── contracts/

│   └── FHEOracle.sol│   └── FHEOracle.sol          # Solidity oracle contract

├── docs/├── docs/

│   ├── INTEGRATION.md│   ├── INTEGRATION.md         # FHE + Blocksense workflow

│   └── mvp-sprint-plan.md│   └── mvp-sprint-plan.md     # 10-week development roadmap

├── examples/├── examples/

│   └── zama_integer_sum.rs│   └── zama_integer_sum.rs    # Rust POC: homomorphic aggregation

├── sdk/├── sdk/

│   └── encrypt.js│   └── encrypt.js             # Node.js adapter: quantize & encrypt

├── .github/workflows/├── .github/workflows/

│   └── ci.yml│   └── ci.yml                 # GitHub Actions CI pipeline

├── .gitignore├── .gitignore                 # Git ignore rules

├── LICENSE├── LICENSE                    # MIT License

└── README.md└── README.md                  # This file

``````



## 🚀 Quick Start## 🚀 Quick Start



```bash### Prerequisites

git clone https://github.com/dharmanan/blocksense-fhe-oracle.git

cd blocksense-fhe-oracle- Node.js 18+ (for SDK)

git checkout dev- Rust 1.70+ (for examples)

- Solidity 0.8.18+ (for contracts)

# Node SDK

cd sdk && npm install### Setup



# Rust examples```bash

cd ../examples && cargo build && cargo testgit clone https://github.com/dharmanan/blocksense-fhe-oracle.git

```cd blocksense-fhe-oracle

git checkout dev

## 🏗️ Architecture

# Node SDK

Data Provider → Quantize & Encrypt → FHE Compute → Threshold Decrypt → On-Chain Settlementcd sdk && npm install



See [`docs/INTEGRATION.md`](docs/INTEGRATION.md) for technical details.# Rust examples

cd ../examples && cargo build

## 📖 Documentation

# Run tests

- [`docs/INTEGRATION.md`](docs/INTEGRATION.md) - FHE integration guidecargo test

- [`docs/mvp-sprint-plan.md`](docs/mvp-sprint-plan.md) - 10-week roadmap```



## 🔧 Development## 🏗️ Architecture



```bashData Provider → Quantize & Encrypt → FHE Compute → Threshold Decrypt (MPC) → On-Chain Finalization

git checkout dev

# Make changes...See [`docs/INTEGRATION.md`](docs/INTEGRATION.md) for detailed workflow.

git commit -m "feat: description"

git push origin dev## 📖 Documentation

```

- [`docs/INTEGRATION.md`](docs/INTEGRATION.md) - Technical integration guide

## 📝 License- [`docs/mvp-sprint-plan.md`](docs/mvp-sprint-plan.md) - 10-week development roadmap



MIT — See [`LICENSE`](LICENSE)## 🔧 Development



**Status**: 🚧 POC PhaseBranch strategy:

- **main**: Production-ready, protected branch
- **dev**: Active development, PR target

```bash
git checkout dev
# Make changes...
git commit -m "feat: description"
git push origin dev
# Open PR for review
```

## 📝 License

MIT — See [`LICENSE`](LICENSE) for details.

---

**Status**: 🚧 POC Phase — Active development
