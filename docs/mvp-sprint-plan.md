# MVP Sprint Plan (10-Week Roadmap)# MVP Sprint Plan (10-Week Roadmap)# MVP Sprint Plan (10-Week Roadmap)# MVP Sprint Plan (10 Hafta)



## Overview



Develop an FHE-based oracle POC for privacy-preserving prediction market aggregation.## Overview



---Develop a proof-of-concept (POC) prediction-market oracle leveraging FHE (Zama/Concrete) for privacy-preserving aggregation of provider predictions.



## Week 1: Discovery & Setup## Overview## Hedef



- [ ] Data schema and quantization rules---

- [ ] Rust + Zama environment

- [ ] Solidity + Hardhat testnetDevelop a proof-of-concept (POC) prediction-market oracle leveraging FHE (Zama/Concrete) for privacy-preserving aggregation of provider predictions.

- [ ] GitHub repo + CI scaffold

- [ ] Architecture documentation## Week 1: Discovery & Setup



**Deliverables**: Setup guide, architecture diagram, schema**Goals**: Team alignment, tool setup, architecture finalizationMinimal but complete FHE oracle POC: data adapter → FHE compute → on-chain finalize



---



## Week 2: Data Pipeline Prototype- [ ] Define exact data schema and quantization rules---



- [ ] Node.js adapter (`sdk/encrypt.js`)- [ ] Set up Rust + Zama Concrete environment

- [ ] Quantization logic

- [ ] Zama public key integration- [ ] Set up Solidity + Hardhat testnet## Haftalık Breakdown

- [ ] Unit tests

- [ ] Prepare GitHub repo with CI/CD scaffold

**Deliverables**: Working adapter, test fixtures

- [ ] Document key assumptions and constraints## Week 1: Discovery & Setup

---



## Week 3: FHE Compute POC

**Deliverables**: Setup guide, architecture diagram, data schema doc**Goals**: Team alignment, tool setup, architecture finalization### **Hafta 1: Keşif & Setup**

- [ ] Zama/Concrete study

- [ ] Rust integer sum (`examples/zama_integer_sum.rs`)

- [ ] Homomorphic addition

- [ ] Threshold comparison---- [ ] Zama/Concrete kütüphaneleri özet ve kurulum



**Deliverables**: FHE compute module, correctness tests



---## Week 2: Data Pipeline Prototype (Adapter + Quantize + Encrypt)- [ ] Define exact data schema and quantization rules- [ ] Solidity oracle kontratı mimari tasarım (dispute flow)



## Week 4: Threshold Decryption**Goals**: Build working adapter that quantizes and encrypts provider submissions



- [ ] Key share generation (3-of-5)- [ ] Set up Rust + Zama Concrete environment- [ ] Quantization standardları belirleme

- [ ] Shamir's secret sharing

- [ ] Decryptor endpoints- [ ] Implement Node.js adapter in `sdk/encrypt.js`

- [ ] Signature aggregation

  - Quantization logic (percent → integer scale)- [ ] Set up Solidity + Hardhat testnet- [ ] Dev ortamı: Rust, Node, Hardhat setup

**Deliverables**: Decryptor scripts, integration tests

  - Zama public key loading

---

  - Ciphertext serialization- [ ] Prepare GitHub repo with CI/CD scaffold- [ ] Dokumentasyon iskelet

## Week 5: Solidity Contract

- [ ] Mock Zama library integration (can use Concrete in later sprint)

- [ ] FHEOracle.sol development

- [ ] Testnet deployment- [ ] Unit tests for quantization edge cases- [ ] Document key assumptions and constraints

- [ ] Contract tests

- [ ] Local endpoint to receive provider submissions

**Deliverables**: Deployed contracts, ABI, tests

**Çıktı:** `docs/ARCHITECTURE.md`, Zama POC repo ready

---

**Deliverables**: Working adapter, sample payloads, unit test suite

## Week 6: End-to-End Integration

**Deliverables**: Setup guide, architecture diagram, data schema doc

- [ ] 5 sample events

- [ ] Provider submissions---

- [ ] FHE aggregation

- [ ] On-chain finalization---



**Deliverables**: E2E test report, event data, logs## Week 3: FHE Compute POC (Integer Sum + Compare)



---**Goals**: Implement homomorphic integer aggregation and threshold evaluation---



## Week 7: Dispute & Jury Flow



- [ ] Jury selection logic- [ ] Study Zama/Concrete integer API### **Hafta 2: Data Pipeline Prototip**

- [ ] Challenge protocol

- [ ] Dispute voting- [ ] Implement pseudo-FHE sum in Rust (`examples/zama_integer_sum.rs`)



**Deliverables**: Dispute flow spec, test cases  - Load ciphertexts from submissions## Week 2: Data Pipeline Prototype (Adapter + Quantize + Encrypt)- [ ] Adapter blueprint (`sdk/encrypt.js` skeleton)



---  - Homomorphic addition (Σ weight_i * CT_i)



## Week 8: SDK & Documentation  - Threshold comparison (CT_aggregate - CT_threshold > 0?)**Goals**: Build working adapter that quantizes and encrypts provider submissions- [ ] Quantization fonksiyonları test et



- [ ] SDK packaging- [ ] Return encrypted difference and metadata

- [ ] Provider guide

- [ ] npm package release- [ ] Unit tests for correctness (compare to plaintext baseline)- [ ] Zama public key import ve first encrypt test

- [ ] Deployment runbook



**Deliverables**: Published SDK, full docs

**Deliverables**: Working FHE compute module, correctness proofs, benchmark notes- [ ] Implement Node.js adapter in `sdk/encrypt.js`- [ ] Mock data sources (5 sample events)

---



## Week 9: Performance Tuning

---  - Quantization logic (percent → integer scale)

- [ ] FHE operation profiling

- [ ] Ciphertext optimization

- [ ] Latency reduction

## Week 4: Threshold Decryption Prototype  - Zama public key loading**Çıktı:** 

**Deliverables**: Performance report

**Goals**: Implement MPC-based decryption (3-of-5 key shares)

---

  - Ciphertext serialization- `sdk/encrypt.js` → CLI ile çalışan encrypt script

## Week 10: Security Review & Demo

- [ ] Generate and distribute 5 key shares from master key

- [ ] Code review

- [ ] Vulnerability assessment- [ ] Implement 3-of-5 threshold scheme (Shamir-like or Zama native)- [ ] Mock Zama library integration (can use Concrete in later sprint)- `examples/sample-data.json` test fixtures

- [ ] Live demo (3-5 events)

- [ ] Known issues documentation- [ ] Mock decryptor endpoints (could be AWS Lambda, Docker containers)



**Deliverables**: Security audit, demo recording- [ ] Signature generation and aggregation- [ ] Unit tests for quantization edge cases- İlk test pass: `npm run test:adapter`



---- [ ] Fault tolerance (2 decryptors can be offline)



## Success Criteria- [ ] Local endpoint to receive provider submissions



- ✅ 5+ events finalized on-chain**Deliverables**: Decryptor setup scripts, signature aggregation logic, integration tests

- ✅ FHE correctness verified

- ✅ Dispute resolution working---

- ✅ < 2 minute latency

- ✅ < $1 per event cost---

- ✅ >80% test coverage

- ✅ No critical security issues**Deliverables**: Working adapter, sample payloads, unit test suite



---## Week 5: Solidity Contract Deployment (Testnet)



## Risks & Mitigations**Goals**: Deploy oracle contract and integrate with finalization flow### **Hafta 3: FHE Compute POC**



| Risk | Mitigation |

|------|-----------|

| API changes | Regular updates, feature flags |- [ ] Complete `contracts/FHEOracle.sol`---- [ ] Zama integer sum example (`examples/zama_integer_sum.rs`)

| Decryptor downtime | Redundancy, 3-of-5 threshold |

| High compute cost | Batching, circuit optimization |  - Result proposal, finalization, dispute, payout logic

| Gas costs | Proof compression, L2 option |

| Provider collusion | Jury arbitration, reputation |  - Access control (admin, proposer, challenger roles)- [ ] Homomorfik addition test (basit: 2 ciphertext ekle)



---- [ ] Deploy to Sepolia/Goerli testnet



## Notes- [ ] Set up contract events and off-chain listeners## Week 3: FHE Compute POC (Integer Sum + Compare)- [ ] Threshold compare logic (CT_sum > threshold?)



- 1-week sprints, adjust per velocity- [ ] Write contract integration tests (Hardhat)

- `dev` for active work, `main` for releases

- Weekly progress sync**Goals**: Implement homomorphic integer aggregation and threshold evaluation- [ ] Sonuç ciphertext serialize/deserialize

- Alpha release after Week 6

**Deliverables**: Deployed contract addresses, ABI, integration test suite



---

- [ ] Study Zama/Concrete integer API**Çıktı:**

## Week 6: End-to-End Integration (5 Sample Events)

**Goals**: Run complete workflow for 5 representative events- [ ] Implement pseudo-FHE sum in Rust (`examples/zama_integer_sum.rs`)- `examples/zama_integer_sum.rs` compiled & tested



- [ ] Create 5 test events (e.g., "Will BTC exceed $40k?", "Will USDC trade below $0.99?")  - Load ciphertexts from submissions- `examples/fhe-test-vectors.json` (CT, weights, expected outputs)

- [ ] 3-5 mock providers submit predictions (via adapter)

- [ ] FHE compute aggregates and compares to threshold  - Homomorphic addition (Σ weight_i * CT_i)

- [ ] Decryptors sign result

- [ ] Submit result to contract and finalize  - Threshold comparison (CT_aggregate - CT_threshold > 0?)---

- [ ] Verify on-chain state and payouts

- [ ] Return encrypted difference and metadata

**Deliverables**: End-to-end test report, sample event data, logs

- [ ] Unit tests for correctness (compare to plaintext baseline)### **Hafta 4: Threshold Decrypt Prototype**

---

- [ ] MPC key split (3-of-5 demo, sharmir's secret sharing)

## Week 7: Dispute & Jury Flow

**Goals**: Implement off-chain dispute resolution mechanism**Deliverables**: Working FHE compute module, correctness proofs, benchmark notes- [ ] Partial decrypt share generation



- [ ] Design jury selection and challenge logic- [ ] Threshold decrypt combiner (3+ shares → plaintext)

- [ ] Implement dispute endpoint and jury voting

- [ ] Create challenge-response protocol---- [ ] Signature + proof generation mock

- [ ] Deploy dispute resolution on-chain (or off-chain arbitration service)

- [ ] Test dispute scenarios (incorrect result, decryptor collusion)



**Deliverables**: Dispute flow spec, test cases, deployment config## Week 4: Threshold Decryption Prototype**Çıktı:**



---**Goals**: Implement MPC-based decryption (3-of-5 key shares)- `src/threshold-decrypt.rs` (or TS adapter)



## Week 8: SDK & Documentation- `examples/decrypt-e2e-test.sh` e2e bash flow

**Goals**: Package and document for external developers

- [ ] Generate and distribute 5 key shares from master key- Security assumptions logged in `SECURITY.md`

- [ ] Create comprehensive README with quickstart

- [ ] Write provider integration guide- [ ] Implement 3-of-5 threshold scheme (Shamir-like or Zama native)

- [ ] Document FHE compute API

- [ ] Publish npm package for adapter (`@blocksense/fhe-oracle-sdk`)- [ ] Mock decryptor endpoints (could be AWS Lambda, Docker containers)---

- [ ] Create Rust crate (if applicable)

- [ ] Write deployment runbook- [ ] Signature generation and aggregation



**Deliverables**: Published SDK, full developer docs, tutorials- [ ] Fault tolerance (2 decryptors can be offline)### **Hafta 5: Solidity Oracle Deploy (Testnet)**



---- [ ] `contracts/FHEOracle.sol` write + review



## Week 9: Performance Tuning**Deliverables**: Decryptor setup scripts, signature aggregation logic, integration tests  - proposeResult, disputeResult, finalizeResult

**Goals**: Optimize latency and compute costs

  - Jury registry (hardcoded whitelist → MVP)

- [ ] Profile FHE operations (bottleneck analysis)

- [ ] Optimize ciphertext size and network transmission---  - Events & emissions

- [ ] Batch multiple events if beneficial

- [ ] Reduce key share generation latency- [ ] Deploy to Sepolia/Goerli

- [ ] Benchmark on staging environment

## Week 5: Solidity Contract Deployment (Testnet)- [ ] ABI export for SDK

**Deliverables**: Performance report, optimization summary, recommendations

**Goals**: Deploy oracle contract and integrate with finalization flow

---

**Çıktı:**

## Week 10: Security Review & Demo

**Goals**: Internal security audit and final demo- [ ] Complete `contracts/FHEOracle.sol`- Contract deployed (address logged in `.env.testnet`)



- [ ] Code review (security + quality)  - Result proposal, finalization, dispute, payout logic- Hardhat test suite: 10+ unit tests green

- [ ] Vulnerability assessment (FHE-specific risks)

- [ ] Penetration testing (network, contract)  - Access control (admin, proposer, challenger roles)- `contracts/FHEOracle.json` ABI

- [ ] Prepare live demo (3-5 events in real-time)

- [ ] Document known limitations and future work- [ ] Deploy to Sepolia/Goerli testnet



**Deliverables**: Security audit report, demo recording, known issues log- [ ] Set up contract events and off-chain listeners---



---- [ ] Write contract integration tests (Hardhat)



## Success Criteria### **Hafta 6: End-to-End (5 Sample Events)**



- ✅ 5+ events successfully aggregated and finalized on-chain**Deliverables**: Deployed contract addresses, ABI, integration test suite- [ ] Adapter → encrypt 5 events

- ✅ All FHE operations compute correctly (verified against plaintext)

- ✅ Dispute resolution tested and working- [ ] FHE compute pipeline run all 5

- ✅ < 2 minute latency (submission to finalization)

- ✅ < $1 per event finalization cost---- [ ] Threshold decrypt + sign results

- ✅ Full test coverage (>80%)

- ✅ No critical security issues- [ ] On-chain finalize batch



---## Week 6: End-to-End Integration (5 Sample Events)- [ ] Verify payout logic (mock)



## Risks & Mitigations**Goals**: Run complete workflow for 5 representative events



| Risk | Mitigation |**Çıktı:**

|------|-----------|

| Zama library API changes | Regular dependency updates, feature flags |- [ ] Create 5 test events (e.g., "Will BTC exceed $40k?", "Will USDC trade below $0.99?")- `test/e2e-flow.test.js` green

| Decryptor availability | Redundant decryptors, 3-of-5 threshold |

| FHE computation cost | Batch operations, optimize circuit depth |- [ ] 3-5 mock providers submit predictions (via adapter)- 5 events fully processed, on-chain finalized

| On-chain gas costs | Compress proofs, use L2 if needed |

| Provider collusion | Jury/arbitration, reputation system |- [ ] FHE compute aggregates and compares to threshold- Gas reports



---- [ ] Decryptors sign result



## Notes- [ ] Submit result to contract and finalize---



- Sprints are 1-week blocks; adjust based on team velocity- [ ] Verify on-chain state and payouts

- Maintain `dev` branch for active work; `main` for releases

- Weekly sync to review progress and blockers### **Hafta 7: Dispute/Jury Flow**

- Consider alpha/beta release after Week 6 for early feedback

**Deliverables**: End-to-end test report, sample event data, logs- [ ] Dispute triggering logic (on-chain)

- [ ] Jury vote interface (`contracts/JuryRegistry.sol`)

---- [ ] Result reversion on jury consensus (mock)

- [ ] Operator reputation / slashing (future)

## Week 7: Dispute & Jury Flow

**Goals**: Implement off-chain dispute resolution mechanism**Çıktı:**

- Dispute flow e2e tested

- [ ] Design jury selection and challenge logic- Jury voting smart contract skeleton

- [ ] Implement dispute endpoint and jury voting- Dispute outcome sample scenarios

- [ ] Create challenge-response protocol

- [ ] Deploy dispute resolution on-chain (or off-chain arbitration service)---

- [ ] Test dispute scenarios (incorrect result, decryptor collusion)

### **Hafta 8: SDK & Docs**

**Deliverables**: Dispute flow spec, test cases, deployment config- [ ] JS/TS SDK publish (`sdk/fhe-oracle-sdk`)

  - `encryptValue(val, key)`, `proposeResult()`, `queryResult()`

---- [ ] Rust SDK skeleton

- [ ] API documentation (OpenAPI/GraphQL mock)

## Week 8: SDK & Documentation- [ ] Integration guide (`docs/INTEGRATION.md` expand)

**Goals**: Package and document for external developers

**Çıktı:**

- [ ] Create comprehensive README with quickstart- npm package: `blocksense-fhe-oracle-sdk` (v0.1.0-alpha)

- [ ] Write provider integration guide- Example apps (3 use cases)

- [ ] Document FHE compute API- README updated

- [ ] Publish npm package for adapter (`@blocksense/fhe-oracle-sdk`)

- [ ] Create Rust crate (if applicable)---

- [ ] Write deployment runbook

### **Hafta 9: Performance Tuning**

**Deliverables**: Published SDK, full developer docs, tutorials- [ ] FHE compute time benchmarks (n=5..50 providers)

- [ ] Threshold decrypt latency

---- [ ] On-chain gas optimization

- [ ] Batch processing exploration

## Week 9: Performance Tuning

**Goals**: Optimize latency and compute costs**Çıktı:**

- `PERFORMANCE.md`: latency tables, recommendations

- [ ] Profile FHE operations (bottleneck analysis)- Optimized contract (gas golfed)

- [ ] Optimize ciphertext size and network transmission- Sample: 10 events < 2 min end-to-end

- [ ] Batch multiple events if beneficial

- [ ] Reduce key share generation latency---

- [ ] Benchmark on staging environment

### **Hafta 10: Security Review & Demo**

**Deliverables**: Performance report, optimization summary, recommendations- [ ] Threat model review

- [ ] Key management audit (shared shard security)

---- [ ] Ciphertext privacy verification

- [ ] Demo recording (5 min walkthrough)

## Week 10: Security Review & Demo- [ ] Handoff docs

**Goals**: Internal security audit and final demo

**Çıktı:**

- [ ] Code review (security + quality)- `SECURITY.md` threat model & mitigations

- [ ] Vulnerability assessment (FHE-specific risks)- Security review checklist (passed)

- [ ] Penetration testing (network, contract)- Demo video + live Sepolia interaction

- [ ] Prepare live demo (3-5 events in real-time)- README production-ready

- [ ] Document known limitations and future work

---

**Deliverables**: Security audit report, demo recording, known issues log

## Milestones

---

| Hafta | Milestone | Status |

## Success Criteria|-------|-----------|--------|

| 2 | Adapter POC | ✓ (hafta 2 sonu) |

- ✅ 5+ events successfully aggregated and finalized on-chain| 4 | FHE compute & decrypt | ✓ (hafta 4 sonu) |

- ✅ All FHE operations compute correctly (verified against plaintext)| 6 | E2E on-chain | ✓ (hafta 6 sonu) |

- ✅ Dispute resolution tested and working| 8 | SDK published | ✓ (hafta 8 sonu) |

- ✅ < 2 minute latency (submission to finalization)| 10 | Security audit + demo | ✓ (hafta 10 sonu) |

- ✅ < $1 per event finalization cost

- ✅ Full test coverage (>80%)## Görev Atama (Örnek)

- ✅ No critical security issues

| Rol | Sorumluluk |

---|-----|------------|

| Backend Engineer | Weeks 1-4 (Rust FHE compute) |

## Risks & Mitigations| Frontend/SDK | Weeks 2, 5-8 (JS adapter, SDK) |

| Solidity Dev | Weeks 5-7 (Smart contracts) |

| Risk | Mitigation || Sec Lead | Week 10 (audit) + ongoing |

|------|-----------|| PM | Planning, stakeholder comms |

| Zama library API changes | Regular dependency updates, feature flags |

| Decryptor availability | Redundant decryptors, 3-of-5 threshold |## Riskler & Mitigations

| FHE computation cost | Batch operations, optimize circuit depth |

| On-chain gas costs | Compress proofs, use L2 if needed || Risk | Mitigation |

| Provider collusion | Jury/arbitration, reputation system ||------|-----------|

| Zama API breaking changes | Pin version, community contact |

---| MPC coordination delay | Async signing, timeout mechanism |

| Gas cost spike | Batch finalize, off-chain state |

## Notes| Jury coordination | Start with admin-only (MVP) |



- Sprints are 1-week blocks; adjust based on team velocity---

- Maintain `dev` branch for active work; `main` for releases

- Weekly sync to review progress and blockers## Sonraki Aşamalar (Post-MVP)

- Consider alpha/beta release after Week 6 for early feedback

- [ ] Multi-chain oracle (Polygon, Arbitrum)
- [ ] Dynamik jury selection
- [ ] Reputation system & slashing
- [ ] Cross-oracle aggregation
- [ ] Mainnet deployment + insurance
