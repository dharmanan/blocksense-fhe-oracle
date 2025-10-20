# MVP Sprint Plan (10-Week Roadmap)

## Overview

Develop an FHE-based oracle POC for privacy-preserving prediction market aggregation.

---

## Week 1: Discovery & Setup

- [ ] Data schema and quantization rules
- [ ] Rust + Zama environment
- [ ] Solidity + Hardhat testnet
- [ ] GitHub repo + CI scaffold
- [ ] Architecture documentation

**Deliverables**: Setup guide, architecture diagram, schema

---

## Week 2: Data Pipeline Prototype

- [ ] Node.js adapter (`sdk/encrypt.js`)
- [ ] Quantization logic
- [ ] Zama public key integration
- [ ] Unit tests

**Deliverables**: Working adapter, test fixtures

---

## Week 3: FHE Compute POC

- [ ] Zama/Concrete study
- [ ] Rust integer sum (`examples/zama_integer_sum.rs`)
- [ ] Homomorphic addition
- [ ] Threshold comparison

**Deliverables**: FHE compute module, correctness tests

---

## Week 4: Threshold Decryption

- [ ] Key share generation (3-of-5)
- [ ] Shamir's secret sharing
- [ ] Decryptor endpoints
- [ ] Signature aggregation

**Deliverables**: Decryptor scripts, integration tests

---

## Week 5: Solidity Contract

- [ ] FHEOracle.sol development
- [ ] Testnet deployment
- [ ] Contract tests

**Deliverables**: Deployed contracts, ABI, tests

---

## Week 6: End-to-End Integration

- [ ] 5 sample events
- [ ] Provider submissions
- [ ] FHE aggregation
- [ ] On-chain finalization

**Deliverables**: E2E test report, event data, logs

---

## Week 7: Dispute & Jury Flow

- [ ] Jury selection logic
- [ ] Challenge protocol
- [ ] Dispute voting

**Deliverables**: Dispute flow spec, test cases

---

## Week 8: SDK & Documentation

- [ ] SDK packaging
- [ ] Provider guide
- [ ] npm package release
- [ ] Deployment runbook

**Deliverables**: Published SDK, full docs

---

## Week 9: Performance Tuning

- [ ] FHE operation profiling
- [ ] Ciphertext optimization
- [ ] Latency reduction

**Deliverables**: Performance report

---

## Week 10: Security Review & Demo

- [ ] Code review
- [ ] Vulnerability assessment
- [ ] Live demo (3-5 events)
- [ ] Known issues documentation

**Deliverables**: Security audit, demo recording

---

## Success Criteria

- ✅ 5+ events finalized on-chain
- ✅ FHE correctness verified
- ✅ Dispute resolution working
- ✅ < 2 minute latency
- ✅ < $1 per event cost
- ✅ >80% test coverage
- ✅ No critical security issues

---

## Notes

- 1-week sprints, adjust per velocity
- `dev` for active work, `main` for releases
- Weekly progress sync
- Alpha release after Week 6
