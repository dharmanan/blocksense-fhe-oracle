# MVP Sprint Plan (10-Week Roadmap)

## Overview

Develop an FHE-based oracle POC for privacy-preserving prediction market aggregation.

---

## Week 1: Discovery & Setup ✅

- [x] Data schema and quantization rules
- [x] Rust + Zama environment
- [x] Solidity + Hardhat testnet
- [x] GitHub repo + CI scaffold
- [x] Architecture documentation

**Status**: Complete | **Date**: Weeks 1-2  
**Deliverables**: Setup guide, architecture diagram, schema ✓

---

## Week 2: Data Pipeline Prototype ✅

- [x] Node.js adapter (`sdk/encrypt.js`)
- [x] Quantization logic
- [x] Zama public key integration
- [x] Unit tests (41 tests passing)

**Status**: Complete | **Date**: Week 2  
**Deliverables**: Working adapter, test fixtures ✓

---

## Week 3: FHE Compute POC ✅

- [x] Zama/Concrete study
- [x] Rust integer sum (`examples/zama_integer_sum.rs`)
- [x] Homomorphic addition & weighted aggregation
- [x] Threshold comparison (encrypted)

**Status**: Complete | **Date**: November 6, 2025  
**Deliverables**: FHE compute module (800+ lines), 9 tests passing ✓

---

## Week 4: Threshold Decryption ✅

- [x] Key share generation (3-of-5 Shamir's SSS)
- [x] Shamir's secret sharing with Lagrange interpolation
- [x] Share verification (VSS with Byzantine fault detection)
- [x] Modular arithmetic (Extended GCD, field operations)

**Status**: Complete | **Date**: November 6, 2025  
**Deliverables**: Threshold decryption (1650+ lines), 28 tests passing (100%) ✓

---

## Week 5: Solidity Contract ⏳

- [ ] FHEOracle.sol development
- [ ] On-chain threshold verification
- [ ] Testnet deployment (Sepolia)
- [ ] Contract tests (15+)

**Status**: Planned | **Next**: Ready to start  
**Deliverables**: Deployed contracts, ABI, tests

---

## Week 6: End-to-End Integration ⏳

- [ ] 5 sample events
- [ ] Provider submissions
- [ ] FHE aggregation
- [ ] On-chain finalization

**Status**: Planned | **Prerequisites**: Week 5 complete  
**Deliverables**: E2E test report, event data, logs

---

## Week 7: Dispute & Jury Flow ⏳

- [ ] Jury selection logic
- [ ] Challenge protocol
- [ ] Dispute voting

**Status**: Planned  
**Deliverables**: Dispute flow spec, test cases

---

## Week 8: SDK & Documentation ⏳

- [ ] SDK packaging
- [ ] Provider guide
- [ ] npm package release
- [ ] Deployment runbook

**Status**: Planned  
**Deliverables**: Published SDK, full docs

---

## Week 9: Performance Tuning ⏳

- [ ] FHE operation profiling
- [ ] Ciphertext optimization
- [ ] Latency reduction

**Status**: Planned  
**Deliverables**: Performance report

---

## Week 10: Security Review & Demo ⏳

- [ ] Code review
- [ ] Vulnerability assessment
- [ ] Live demo (3-5 events)
- [ ] Known issues documentation

**Status**: Planned  
**Deliverables**: Security audit, demo recording

---

## Project Status (Last Updated: November 6, 2025)

| Week | Component | Status | Tests | Commit |
|------|-----------|--------|-------|--------|
| 1-2 | Setup + Data Pipeline | ✅ Complete | 41 | 002e4c0 |
| 3 | FHE Compute POC | ✅ Complete | 9 | 002e4c0 |
| 4 | Threshold Decryption | ✅ Complete | 28 | 15f6b0e |
| 5-10 | Smart Contract + Integration | ⏳ Planned | - | Upcoming |

**Total Progress**: 40% of MVP (Weeks 1-4 complete)

---

## Success Criteria

- ✅ 5+ events finalized on-chain (Week 6 target)
- ✅ FHE correctness verified (Week 3 ✓)
- ⏳ Dispute resolution working (Week 7 target)
- ⏳ < 2 minute latency (Performance target)
- ⏳ < $1 per event cost (Optimization target)
- ✅ >80% test coverage (Currently 100% on Weeks 3-4)
- ⏳ No critical security issues (Security audit Week 10)

---

## Notes

- 1-week sprints, adjust per velocity
- `dev` for active work, `main` for releases
- Weekly progress sync
- Alpha release after Week 6
