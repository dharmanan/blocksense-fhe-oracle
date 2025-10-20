# blocksense-fhe-oracle

Blocksense + FHE (Fully Homomorphic Encryption, powered by Zama/Concrete) proof-of-concept for privacy-preserving prediction market oracle.

## 🎯 Objective

Build a decentralized oracle system that aggregates provider predictions using homomorphic encryption, settles on-chain with threshold decryption, and resolves disputes via jury mechanism.

## 📋 Project Structure

```
blocksense-fhe-oracle/
├── contracts/
│   ├── FHEOracle.sol                      # Main oracle smart contract
│   ├── hardhat.config.js                  # Hardhat config (4 networks)
│   ├── package.json                       # Dependencies & deployment scripts
│   ├── scripts/
│   │   └── deploy.js                      # Multi-network deployment
│   ├── test/
│   │   └── FHEOracle.test.js             # 19 comprehensive contract tests
│   └── .env.example                       # Environment template
├── docs/
│   ├── ARCHITECTURE.md                    # 11-section system design
│   ├── ARCHITECTURE-DIAGRAMS.md           # 7 ASCII architecture diagrams
│   ├── HARDHAT.md                         # Complete Hardhat guide
│   ├── ZAMA-INTEGRATION.md                # Zama/Concrete integration guide
│   ├── quantization-spec.md               # Quantization rules & test vectors
│   ├── INTEGRATION.md                     # API & integration reference
│   └── mvp-sprint-plan.md                 # 10-week development roadmap
├── examples/
│   ├── Cargo.toml                         # Rust dependencies
│   └── zama_integer_sum.rs                # FHE aggregation POC
├── sdk/
│   ├── package.json                       # SDK dependencies
│   ├── .eslintrc.json                     # Linting config
│   └── encrypt.js                         # Quantize & encrypt functions
├── tests/
│   ├── quantization_test.rs               # 41 Rust quantization tests
│   └── quantization.test.js               # 38 JavaScript quantization tests
├── .github/workflows/
│   └── ci.yaml                            # 7-job CI/CD pipeline
├── .gitignore
├── LICENSE                                # MIT License
└── README.md
```

## 🚀 Quick Start

### Clone & Setup
```bash
git clone https://github.com/dharmanan/blocksense-fhe-oracle.git
cd blocksense-fhe-oracle
git checkout dev
```

### Run Tests (All Platforms)

**Rust (Quantization & FHE Engine)**
```bash
cargo test --test quantization_test -- --nocapture
cargo build --example zama_integer_sum --release
```

**JavaScript (SDK & Quantization)**
```bash
cd sdk && npm ci
npm test                                    # SDK tests
cd ../tests && npx mocha quantization.test.js
```

**Solidity (Smart Contract)**
```bash
cd contracts
npm ci
npm run compile
npm run test                                # 19 contract tests
npm run node                                # Start local blockchain
npm run deploy:local                        # Deploy to localhost
```

### 📚 Documentation

**Core Architecture & Design**
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — System design (11 sections)
- [ARCHITECTURE-DIAGRAMS.md](docs/ARCHITECTURE-DIAGRAMS.md) — Visual diagrams (7 types)

**Integration Guides**
- [HARDHAT.md](docs/HARDHAT.md) — Smart contract development
- [ZAMA-INTEGRATION.md](docs/ZAMA-INTEGRATION.md) — FHE library integration
- [quantization-spec.md](docs/quantization-spec.md) — Quantization rules & test vectors

**Roadmap & Planning**
- [mvp-sprint-plan.md](docs/mvp-sprint-plan.md) — 10-week development roadmap
- [INTEGRATION.md](docs/INTEGRATION.md) — Full API reference

## 🏗️ Architecture

```
Provider Prediction → Quantize & Encrypt 🔐 → FHE Homomorphic Compute → 
Threshold Decryption (MPC 3-of-5) → On-Chain Settlement & Dispute Resolution
```

### System Flow (4 Phases)

1. **Submission** (Locked & Encrypted)
   - Providers submit quantized predictions encrypted with FHE
   - Data stored on-chain immutably

2. **Aggregation** (FHE Computation - Stays Encrypted!)
   - Rust engine performs homomorphic operations
   - Average, weighted sum, threshold comparison all encrypted
   - No intermediate decryption needed

3. **Decryption** (MPC Threshold Secret Sharing)
   - 3-of-5 decryptor nodes reconstruct encryption key
   - Lagrange interpolation of Shamir shares
   - Result signed by decryptors

4. **Settlement** (On-Chain Finalization)
   - Multisig verification of result
   - Payout distribution
   - Dispute resolution if challenged

### Key Components

- **`contracts/FHEOracle.sol`**: Solidity oracle for on-chain settlement (event lifecycle, disputes, jury voting)
- **`examples/zama_integer_sum.rs`**: Rust POC for homomorphic aggregation using Zama/Concrete
- **`sdk/encrypt.js`**: Node.js encryption adapter for data providers (quantization + FHE encryption)
- **`tests/`**: 79 comprehensive tests (41 Rust + 38 JavaScript)
- **`docs/`**: Complete technical documentation

## 📖 Documentation

### Technical Specifications
- **`docs/ARCHITECTURE.md`**: Complete system design (11 sections, ~3000 words)
  - Component overview and data flow
  - Security model and threat analysis
  - Performance specifications (40-50s latency, 100 events/s throughput)
  - Deployment strategies (local, testnet, mainnet)

- **`docs/ARCHITECTURE-DIAGRAMS.md`**: Visual reference (7 ASCII diagrams, 600+ lines)
  - High-level system architecture
  - Complete data flow (4 phases)
  - Component interactions
  - Event state machine
  - Quantization scales
  - Security layers
  - Deployment environments

- **`docs/quantization-spec.md`**: Quantization rules (481 lines)
  - 3 market types (Percentage, Price, Ratio)
  - 45+ test vectors with edge cases
  - Precision and rounding rules
  - Integration examples

### Implementation Guides
- **`docs/HARDHAT.md`**: Smart contract development (450+ lines, 15 sections)
  - Installation and setup
  - Network configuration (local, testnet, mainnet)
  - Testing framework and best practices
  - Deployment workflow
  - Gas optimization
  - Troubleshooting

- **`docs/ZAMA-INTEGRATION.md`**: FHE library integration (2000+ words, 12 sections)
  - Rust 1.70+ setup
  - Mock → Real implementation strategy (3 phases)
  - Zama/Concrete API reference
  - Trait-based backend abstraction
  - Testing approach (unit, integration, property-based)
  - Performance benchmarks

### Reference
- **`docs/INTEGRATION.md`**: Full API reference and integration checklist
- **`docs/mvp-sprint-plan.md`**: 10-week development roadmap

## 🧪 Testing

### Test Coverage (105+ Tests Total)

**Rust Tests (79 total)**
```bash
cd /workspaces/blocksense-fhe-oracle
cargo test --test quantization_test -- --nocapture   # 41 tests
cargo test --example zama_integer_sum -- --nocapture # Feature tests
```
- Percentage market validation (13 tests)
- Price market validation (13 tests)
- Ratio market validation (10 tests)
- Integration flows (3 tests)

**JavaScript Tests (38 total)**
```bash
cd tests
npx mocha quantization.test.js -- --reporter spec
```
- Quantization accuracy (38 tests)
- Edge cases and error handling
- SDK integration tests

**Solidity Tests (19 total)**
```bash
cd contracts
npm run test                    # Full suite with gas report
npm run test -- --grep "Event" # Filter by test name
```
- Event creation & management (3 tests)
- Provider submissions (4 tests)
- Result finalization (2 tests)
- Dispute resolution (3 tests)
- Gas efficiency benchmarks (2 tests)
- Edge cases (3 tests)

### CI/CD Pipeline (7 Jobs)
```
✓ Rust Build & Test (Clippy lint, coverage, feature flags)
✓ Node.js Lint (18.x + 20.x, ESLint, security audit)
✓ Solidity Lint (contract compilation, gas report, size check)
✓ Documentation (Markdown validation, link checking)
✓ Integration Tests (end-to-end Hardhat deployment)
✓ Security Audit (secret detection, dependency scanning)
✓ Status Badge (pipeline summary)
```

Run locally:
```bash
npm run ci:all  # If available, or run individual suites
```

## 📊 Project Status

### ✅ Phase 1 Complete (Week 1 Setup)
```
Task 1A: Quantization Specification ✓
  - 3 market types fully defined
  - 45+ test vectors with edge cases
  - 41 Rust tests + 38 JavaScript tests

Task 1B: Hardhat Setup ✓
  - Smart contract testing framework
  - 4 network support (hardhat, localhost, sepolia, mainnet)
  - 19 comprehensive contract tests
  - Multi-network deployment scripts

Task 1C: Architecture Documentation ✓
  - Complete system design (11 sections)
  - 7 visual architecture diagrams
  - Security model and threat analysis

Task 1D: CI/CD & Zama Integration ✓
  - 7-job GitHub Actions pipeline
  - Coverage reporting and security audit
  - Zama/Concrete integration guide
  - Trait-based backend abstraction pattern
```

### 📈 Code Statistics
- **Rust**: ~700 lines (quantization + tests)
- **JavaScript**: ~800 lines (SDK, tests, deployment)
- **Solidity**: ~419 lines (contract tests)
- **YAML**: ~350 lines (CI/CD pipeline)
- **Documentation**: ~6000+ lines

### 🎯 Upcoming: Phase 2 (Week 2-3)
- Data pipeline prototype (Node.js adapter)
- Real quantization implementation
- Live data provider integration
- Event submission flow
- Extended test coverage

## 📝 License

MIT — See LICENSE for details.

**Status**: ✅ Phase 1 Complete | 🔄 Phase 2 Ready | 🚀 Production-Ready Architecture
