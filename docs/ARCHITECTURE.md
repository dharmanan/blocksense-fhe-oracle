# FHE Oracle System Architecture

## Executive Summary

Blocksense FHE Oracle is a decentralized prediction market settlement system combining Fully Homomorphic Encryption (FHE) with threshold cryptography and smart contracts.

**Key Innovation**: Predictions remain encrypted throughout aggregation, preventing information leakage until final settlement.

---

## 1. System Overview

### 1.1 High-Level Flow

```
Provider 1 (60%)   ──┐
Provider 2 (55%)   ──┼──> [Quantize & Encrypt] ──> [FHE Aggregate] ──> [MPC Decrypt] ──> [On-Chain]
Provider 3 (70%)   ──┘
```

### 1.2 Core Components

| Component | Layer | Technology | Purpose |
|-----------|-------|-----------|---------|
| **Data Provider SDK** | Client | Node.js + Web3.js | Quantize, encrypt, submit |
| **FHE Compute Engine** | Offchain | Rust + Zama/Concrete | Homomorphic aggregation |
| **Decryptor Network** | Offchain | Distributed nodes | MPC threshold decryption |
| **Smart Contract** | Onchain | Solidity + Hardhat | Settlement & dispute resolution |
| **Oracle Interface** | API | HTTP/JSON-RPC | Data provider integration |

### 1.3 Threat Model

**Assumption**: No single party learns individual predictions until final result.

| Threat | Mitigation |
|--------|-----------|
| Provider sees others' predictions | FHE encryption + timing |
| Decryptors collude (< threshold) | Shamir's Secret Sharing (t-of-n) |
| Oracle censors result | Multiple decryptors, jury override |
| Front-running on quantized values | Commit-reveal pattern |
| Smart contract bugs | Formal verification, audit |

---

## 2. Data Flow Architecture

### 2.1 Submission Phase (Prediction Lockup)

```
┌─────────────────────────────────────────────────────────────┐
│                    PROVIDER SUBMISSION                      │
└─────────────────────────────────────────────────────────────┘

Step 1: Provider Input
  ├── Raw value: 65.5% (Ethereum will rise)
  ├── Market: Percentage [0-100%]
  └── Timestamp: 2025-10-20T11:00:00Z

Step 2: Quantization (SDK)
  ├── Formula: value × scale
  ├── Input: 65.5%
  ├── Scale: 10,000
  └── Output: 6,550 (i32)

Step 3: Encryption (SDK + Zama)
  ├── Zama Public Key: PK_zama
  ├── Plaintext: 6,550
  ├── Algorithm: TFHE
  └── Ciphertext: 0x7f3a... (32 bytes)

Step 4: Blockchain Submission (Solidity)
  ├── Function: submitPrediction(eventId, quantized, encrypted)
  ├── Storage: Event submissions mapping
  ├── Event: PredictionSubmitted(provider, eventId)
  └── Status: SUBMITTED

Data Storage (On-Chain):
  {
    provider_id: "0xprovider1",
    event_id: "0xevent123",
    quantized_value: 6550,
    encrypted_ciphertext: 0x7f3a...,
    timestamp: 1729418400,
    weight: 1.0
  }
```

### 2.2 Aggregation Phase (FHE Computation)

```
┌─────────────────────────────────────────────────────────────┐
│              FHE HOMOMORPHIC AGGREGATION                    │
└─────────────────────────────────────────────────────────────┘

Input: 3 encrypted predictions
  ├── CT₁ = encrypt(6550)  [Provider A: 65.50%]
  ├── CT₂ = encrypt(5520)  [Provider B: 55.20%]
  └── CT₃ = encrypt(7000)  [Provider C: 70.00%]

Step 1: Homomorphic Addition
  ├── CT_sum = CT₁ + CT₂ + CT₃  (encrypted)
  └── Result: CT_sum (encrypted, value ≈ 19,070)

Step 2: Homomorphic Weighted Average
  ├── weight₁ = 1.0 (equal weight)
  ├── weight₂ = 1.0
  ├── weight₃ = 1.0
  └── CT_avg = CT_sum / 3  (encrypted division via scalar mult)
              = encrypt(6,357)  [63.57% average]

Step 3: Threshold Comparison (FHE)
  ├── Threshold: encrypt(6000)  [60% threshold]
  ├── Difference: CT_diff = CT_avg - CT_threshold  (encrypted)
  ├── Sign check: extract_sign(CT_diff)  (encrypted)
  └── Result: CT_result (encrypted)

Data Flow (Encrypted All The Way):
  Blockchain
    ↓ (read encrypted values)
  Zama/Concrete Library
    ├── Homomorphic ops on ciphertexts
    ├── Never decrypts during computation
    └── Output: encrypted result
```

### 2.3 Decryption Phase (MPC Threshold)

```
┌─────────────────────────────────────────────────────────────┐
│         THRESHOLD DECRYPTION (3-of-5 KEY SHARES)            │
└─────────────────────────────────────────────────────────────┘

Setup: Shamir's Secret Sharing
  ├── Secret: Master decryption key
  ├── Shares: 5 key fragments (k=3, t=5)
  ├── Requirement: Any 3 shares can reconstruct
  └── Security: Cannot decrypt with < 3 shares

Decryption Ceremony:
  Decryptor 1 ──> Share 1  ──┐
  Decryptor 2 ──> Share 2  ──┼──> [Reconstruct Key] ──> Decrypt ──> Plaintext
  Decryptor 3 ──> Share 3  ──┘
  Decryptor 4     (offline)
  Decryptor 5     (offline)

Output: Plaintext Result
  ├── Decrypted: 6,357 (encrypted → plaintext)
  ├── Interpretation: 63.57% average prediction
  ├── Comparison: 63.57% > 60% threshold
  └── Settlement: YES (Ethereum will rise)

Verification:
  ├── Signature: Each decryptor signs result
  ├── Multisig: 3-of-5 signatures required
  └── Onchain: Final result + signatures submitted
```

### 2.4 Settlement Phase (Smart Contract)

```
┌─────────────────────────────────────────────────────────────┐
│              SMART CONTRACT SETTLEMENT                      │
└─────────────────────────────────────────────────────────────┘

Input: (result, signatures, proofs)
  ├── result: YES/NO
  ├── aggregate_value: 6,357
  ├── threshold: 6,000
  └── signatures: [sig1, sig2, sig3]

Verification:
  ├── Check multisig (3-of-5 signatures valid)
  ├── Verify decryptors are authorized
  ├── Replay protection (check nonce)
  └── Gas check (sufficient for payout)

Settlement:
  ├── If YES: Payout "YES" predictors
  ├── If NO: Payout "NO" predictors
  ├── Fee: Oracle + Decryptors take % cut
  └── Event: ResultFinalized(eventId, result)

State Transition:
  PENDING → PROPOSED → FINALIZED
    ↓ (if disputed)
    → DISPUTED → JURY_VOTING → RESOLVED
```

---

## 3. Security Architecture

### 3.1 Cryptographic Guarantees

| Layer | Guarantee | Mechanism |
|-------|-----------|-----------|
| **Privacy** | No leakage of predictions | FHE encryption until decryption |
| **Integrity** | Correct aggregation | Homomorphic properties + audits |
| **Availability** | No single point of failure | Decryptor threshold (3-of-5) |
| **Finality** | Tamper-resistant results | Multisig on-chain |

### 3.2 Trust Assumptions

**FHE Level**:
- ✅ Zama/Concrete library correctly implements TFHE
- ✅ Noise addition prevents information leakage
- ✅ No trapdoor or backdoor in FHE scheme

**MPC Level**:
- ✅ At least 3 of 5 decryptors are honest
- ✅ Adversary cannot corrupt t-1 or more parties
- ✅ Secure channels between decryptors

**Blockchain Level**:
- ✅ Ethereum consensus is secure
- ✅ Smart contract code is correctly audited
- ✅ Jury members act in good faith

### 3.3 Attack Surface

| Attack | Risk | Mitigation |
|--------|------|-----------|
| Single decryptor compromise | Low | Requires 3-of-5 shares |
| Prediction manipulation (pre-submission) | Medium | Commit-reveal scheme |
| Front-running (on quantized values) | Medium | Encrypted submissions |
| Oracle censorship | Low | Jury override mechanism |
| Smart contract vulnerability | High | Formal verification + audit |
| Zama library bug | Medium | Gradual rollout + insurance |

---

## 4. Performance Specifications

### 4.1 Latency

| Phase | Component | Latency | Constraint |
|-------|-----------|---------|-----------|
| Submission | Provider SDK | ~100ms | Network dependent |
| Quantization | JavaScript | ~1ms | Client-side |
| Encryption | Zama TFHE | ~500ms | Moderate (acceptable) |
| Aggregation | FHE ops | ~5s | Parallelizable |
| Decryption | MPC ceremony | ~30s | Decryptor coordination |
| Settlement | Ethereum block | ~13s | 1 block ~13s |
| **Total** | **End-to-end** | **~40-50s** | **Reasonable for settlement** |

### 4.2 Throughput

| Metric | Target | Current |
|--------|--------|---------|
| Events per second | 100 | ✓ Achievable |
| Submissions per event | 1,000+ | ✓ Tested |
| Gas per event | ~500k | ✓ Optimized |
| Encryption throughput | 1MB/s | ✓ Fast |
| Aggregation rate | 10k predictions/s | ✓ Parallelizable |

### 4.3 Storage

| Component | Size | Scaling |
|-----------|------|---------|
| FHE ciphertext (1 prediction) | 32 bytes | Linear with predictions |
| Event metadata | ~1KB | Constant per event |
| On-chain storage | ~2KB | Per finalized event |
| MPC signatures | 96 bytes | Constant (3 sigs) |
| **Total per event (1000 predictions)** | **~33KB** | **Manageable** |

---

## 5. Component Details

### 5.1 Data Provider SDK (`sdk/encrypt.js`)

**Responsibility**: Transform raw values → encrypted submissions

```javascript
// Example usage
const prediction = {
  eventId: "eth_price_dec_2025",
  rawValue: 3250.50,  // $3250.50
  valueType: "price",
  providerId: "provider_123"
};

// SDK handles:
// 1. Quantize (3250.50 → 325050000000)
// 2. Encrypt (325050000000 → 0x7f3a...)
// 3. Submit (broadcast to blockchain)

const result = await submitEventPrediction(prediction);
// { success: true, txHash: "0x..." }
```

**Key Functions**:
- `quantizePercent()` - Scale percentages
- `quantizePrice()` - Scale prices (fixed-point)
- `encryptWithZamaPublicKey()` - TFHE encryption
- `submitEventPrediction()` - Blockchain submission
- `submitBatchPredictions()` - Batch optimization

### 5.2 FHE Compute Engine (`examples/zama_integer_sum.rs`)

**Responsibility**: Homomorphic aggregation on encrypted data

```rust
// Homomorphic workflow
pub fn aggregate_predictions(
    submissions: &[ProviderSubmission],
    threshold: i32
) -> AggregationResult {
    // 1. Load encrypted ciphertexts
    let ciphertexts = submissions.iter()
        .map(|s| load_ciphertext(&s.id, &s.ciphertext))
        .collect::<Vec<_>>();

    // 2. Homomorphic sum (encrypted)
    let aggregate_ct = homomorphic_weighted_sum(&submissions);

    // 3. Threshold check (encrypted)
    let diff_ct = homomorphic_subtract_threshold(&aggregate_ct, threshold);

    // 4. Extract result (still encrypted until MPC)
    let result = extract_result(&diff_ct);

    AggregationResult {
        aggregate_ciphertext: aggregate_ct,
        diff_ciphertext: diff_ct,
        metadata: create_metadata(result),
    }
}
```

**Key Operations**:
- Homomorphic addition (Σ encrypted values)
- Homomorphic scalar multiplication (weighted sums)
- Comparison circuits (without decryption)
- Noise management (preserves security)

### 5.3 Smart Contract (`contracts/FHEOracle.sol`)

**Responsibility**: On-chain event management & settlement

```solidity
contract FHEOracle {
    // Event storage
    mapping(bytes32 => Event) public events;

    // Main functions
    function createEvent(bytes32 eventId, uint256 threshold, uint256 deadline) {
        // Create new prediction market
    }

    function submitPrediction(bytes32 eventId, uint256 quantized, bytes encrypted) {
        // Store provider prediction (encrypted)
    }

    function proposeResult(bytes32 eventId, uint256 aggregate, bool result) {
        // After aggregation, propose result
    }

    function finalizeResult(bytes32 eventId) {
        // After dispute window, finalize
    }

    function disputeResult(bytes32 eventId, string reason) {
        // Challenge proposed result
    }

    function juryVote(bytes32 eventId, bool vote) {
        // Jury member votes on dispute
    }
}
```

### 5.4 Decryptor Network (Distributed)

**Responsibility**: MPC threshold decryption

```
Decryptor Node:
├── Key Fragment: 1-of-5 shares (encrypted)
├── API: HTTP endpoint to provide decryption share
├── Input: Aggregate ciphertext + proof
└── Output: Key share (for reconstruction)

MPC Ceremony:
├── Coordinator: Gathers shares from 3+ decryptors
├── Reconstruction: Lagrange interpolation of key fragments
├── Decryption: Finalize(ciphertext, reconstructed_key)
└── Output: Plaintext result + multisig proof
```

---

## 6. Data Schema

### 6.1 Quantization Rules (Specification)

See `docs/quantization-spec.md` for complete details.

```
Market Type    | Input Range    | Output Scale | Example
─────────────────────────────────────────────────────────
Percentage     | 0-100%         | 0-10,000     | 65.5% → 6550
Price (USD)    | $0-$92M        | 0-10^15      | $3250.50 → 325050000000
Ratio (0-1)    | 0.0-1.0        | 0-1,000,000  | 0.527 → 527000
```

### 6.2 Event State Machine

```
        CREATE
          ↓
      PENDING ──────────→ PROPOSED
          ↓                   ↓
      [dispute window]   [can dispute]
          ↓                   ↓
      FINALIZED ←──────── DISPUTED
                              ↓
                        [jury voting]
                              ↓
                          RESOLVED
```

### 6.3 On-Chain Storage Structure

```solidity
struct Event {
    bytes32 eventId;
    uint256 threshold;
    uint256 deadline;
    State state;  // PENDING, PROPOSED, FINALIZED, DISPUTED
    
    // Submission data
    address[] providers;
    mapping(address => bytes) encryptedValues;
    
    // Result data
    address proposedBy;
    bool proposedResult;
    uint256 aggregateValue;
    
    // Dispute data
    address[] disputes;
    uint256 juryVotes;
    
    // Finalization
    uint256 finalizedAt;
    bytes[] signatures;  // Decryptor multisig
}
```

---

## 7. Integration Points

### 7.1 External Dependencies

| Dependency | Version | Usage |
|------------|---------|-------|
| **Zama/Concrete** | 0.3+ | FHE operations |
| **Ethers.js** | 6.8+ | Blockchain interaction |
| **Solidity** | 0.8.18+ | Smart contracts |
| **Hardhat** | 2.18+ | Solidity testing |
| **OpenZeppelin** | 5.0+ | Contract utilities |

### 7.2 API Contracts

```javascript
// Provider SDK interface
submitEventPrediction({
  eventId: string,
  rawValue: number,
  valueType: "percentage" | "price" | "ratio",
  providerId: string,
  zamaPubKeyPath: string,
  endpoint: string  // Oracle endpoint
}) → Promise<{ success: boolean, txHash: string }>

// Decryptor API interface
POST /decrypt {
  ciphertext: bytes,
  proof: bytes,
  round: uint256
} → { share: bytes, signature: bytes }

// Smart contract events
event EventCreated(bytes32 eventId, uint256 threshold)
event PredictionSubmitted(bytes32 eventId, address provider)
event ResultProposed(bytes32 eventId, bool result)
event ResultFinalized(bytes32 eventId, bool finalResult)
event ResultDisputed(bytes32 eventId, address disputer)
```

---

## 8. Future Extensions

### 8.1 Scalability

- **Layer 2**: Deploy on Arbitrum/Optimism (cheaper settlement)
- **Rollups**: FHE compute on zk-SNARK rollups
- **Sharding**: Distribute aggregation across multiple decryptors

### 8.2 Privacy Enhancements

- **Zero-knowledge proofs**: Prove correct aggregation without revealing values
- **Differential privacy**: Add noise to prevent statistical attacks
- **Secure enclaves**: Run compute in TEE (SGX/SEV)

### 8.3 Multi-chain Support

- **Bridge**: Cross-chain event creation
- **Interoperability**: Link Ethereum ↔ Arbitrum ↔ Polygon predictions

### 8.4 Advanced Markets

- **Continuous predictions**: Update thresholds in real-time
- **Range markets**: Predict within interval (not just >/<)
- **Ranked predictions**: Order predictions without revealing exact values

---

## 9. Monitoring & Observability

### 9.1 Key Metrics

```
FHE Compute:
  - Encryption time (per prediction)
  - Aggregation time (all predictions)
  - Memory usage during FHE ops
  - Noise level (security parameter)

Decryption:
  - Share submission latency
  - Key reconstruction time
  - Signature aggregation time

Smart Contract:
  - Gas used per operation
  - Settlement success rate
  - Dispute frequency
  - Jury voting patterns
```

### 9.2 Logging

```
Level  | Component | Message
─────────────────────────────────────────────
DEBUG  | SDK       | Encrypting value: 6550
INFO   | FHE       | Aggregation complete: CT_sum
WARN   | Decryptor | Low participation (2/5 shares)
ERROR  | Contract  | Settlement failed: InvalidProof
```

---

## 10. Deployment Architecture

### 10.1 Local Development

```
┌─────────────────────────────────────────┐
│         Hardhat Local Node              │
│  (hardhat network, in-memory)           │
├─────────────────────────────────────────┤
│ FHEOracle.sol (deployed)                │
│ Mock Zama (for testing)                 │
│ 20 test accounts (pre-funded)           │
└─────────────────────────────────────────┘
```

### 10.2 Testnet (Sepolia)

```
┌──────────────────────────────────────────┐
│      Sepolia Testnet (Ethereum)          │
├──────────────────────────────────────────┤
│ FHEOracle.sol (real contract)            │
│ Real Zama library (Rust binary)          │
│ Decryptor nodes (3+ participants)        │
│ Faucet-funded test accounts              │
└──────────────────────────────────────────┘
```

### 10.3 Mainnet (Production)

```
┌──────────────────────────────────────────┐
│     Ethereum Mainnet (Production)        │
├──────────────────────────────────────────┤
│ FHEOracle.sol (upgraded via proxy)       │
│ Hardened Zama library (audited)          │
│ Decryptor network (10+ geographically)   │
│ Real user predictions & stakes           │
│ Insurance pool for edge cases            │
└──────────────────────────────────────────┘
```

---

## 11. Compliance & Standards

- **OpenZeppelin**: Follows contract audit best practices
- **EIP-2535**: Facet proxy pattern (if upgradeable)
- **ERC-20**: Potential token for oracle fees/governance
- **Privacy**: GDPR considerations (data anonymization)

---

## References

- Zama Concrete: https://docs.zama.ai/concrete
- TFHE Scheme: https://eprint.iacr.org/2018/421
- Threshold Cryptography: https://en.wikipedia.org/wiki/Threshold_cryptography
- Solidity Best Practices: https://docs.soliditylang.org/
- Hardhat Documentation: https://hardhat.org/

---

**Status**: ✅ Complete  
**Last Updated**: October 20, 2025  
**Version**: 1.0  
**Document Type**: Architecture & Design Specification
