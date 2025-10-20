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
│   └── zama_integer_sum.rs
├── sdk/
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
cd ../sdk && npm install
```

## 🏗️ Architecture

Data Provider → Quantize & Encrypt → FHE Compute → Threshold Decrypt → On-Chain Settlement

See `docs/INTEGRATION.md` for technical details and `docs/mvp-sprint-plan.md` for roadmap.

## 📝 License

MIT — See LICENSE for details.

**Status**: 🚧 POC Phase


## 🏗️ Architecture

Data Provider → Quantize & Encrypt → FHE Compute → Threshold Decrypt → On-Chain Settlement

See `docs/INTEGRATION.md` for technical details and `docs/mvp-sprint-plan.md` for roadmap.

## 📝 License

MIT — See LICENSE for details.

**Status**: 🚧 POC Phase
=======
# blocksense-fhe-oracle
>>>>>>> origin/main
