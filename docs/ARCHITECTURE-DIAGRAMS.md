# Architecture Diagrams

## 1. High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    BLOCKSENSE FHE ORACLE                        │
│                  Privacy-Preserving Settlement                  │
└─────────────────────────────────────────────────────────────────┘

                           CLIENT LAYER
                    ┌────────────────────┐
                    │ Data Providers     │
                    │ (Individual users) │
                    └────────┬───────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
    ┌────────┐          ┌────────┐         ┌────────┐
    │SDK     │          │SDK     │         │SDK     │
    │encrypt │          │encrypt │         │encrypt │
    │.js     │          │.js     │         │.js     │
    └───┬────┘          └───┬────┘         └───┬────┘
        │ quantize          │ quantize         │ quantize
        │ encrypt           │ encrypt          │ encrypt
        └───────┬───────────┼───────────────────┘
                │           │
         ┌──────▼───────────▼──────┐
         │   BLOCKCHAIN LAYER      │
         │   Ethereum / Testnet    │
         │                         │
         │  ┌──────────────────┐  │
         │  │ FHEOracle.sol    │  │
         │  ├──────────────────┤  │
         │  │ - Event storage  │  │
         │  │ - Submission log │  │
         │  │ - Settlement     │  │
         │  └──────────────────┘  │
         └──────┬─────────────────┘
                │
    ┌───────────┼───────────────────────────┐
    │           │                           │
    ▼           ▼                           ▼
┌────────┐  ┌──────────────┐        ┌──────────────┐
│FHE     │  │Decryptor     │        │Jury / Oracle │
│Compute │  │Network       │        │Dispute Res   │
│Engine  │  │(MPC 3-of-5)  │        │              │
│(Rust)  │  └──────────────┘        └──────────────┘
└────────┘

            OFFCHAIN AGGREGATION LAYER
    - Homomorphic encryption
    - FHE computations
    - MPC threshold decryption
    - Result signing
```

---

## 2. Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                      PREDICTION MARKET EVENT                    │
│                   "ETH Price > $3000 by Dec 31?"                │
└─────────────────────────────────────────────────────────────────┘

PHASE 1: SUBMISSION (Locked & Encrypted)
══════════════════════════════════════════════════════════════════

Provider A (Alice)          Provider B (Bob)         Provider C (Charlie)
    │                           │                         │
    ├─ Raw input: 65%          ├─ Raw input: 55%        ├─ Raw input: 70%
    │  (YES, price will rise)   │  (NO, too uncertain)   │  (YES, very likely)
    │                           │                         │
    ├─ Quantize: 6550          ├─ Quantize: 5500       ├─ Quantize: 7000
    │                           │                         │
    ├─ Encrypt 🔐              ├─ Encrypt 🔐           ├─ Encrypt 🔐
    │  CT_A = E(6550)           │  CT_B = E(5500)       │  CT_C = E(7000)
    │                           │                         │
    └──────────────┬────────────┴───────────┬────────────┘
                   │                       │
                   │ Submit to blockchain  │
                   └───────────┬───────────┘
                               │
                       ┌───────▼────────┐
                       │ Blockchain:    │
                       │ Event {        │
                       │   CT_A, CT_B   │
                       │   CT_C stored  │
                       │ }              │
                       └────────────────┘


PHASE 2: AGGREGATION (FHE Computation - Stays Encrypted!)
══════════════════════════════════════════════════════════════════

                        FHE COMPUTE ENGINE
                        (Rust + Zama)

Input (encrypted):
    CT_A 🔐 (value: ?)
    CT_B 🔐 (value: ?)
    CT_C 🔐 (value: ?)

              ↓ Homomorphic Addition
        
    CT_SUM = CT_A + CT_B + CT_C (all encrypted!)
    CT_SUM 🔐 (total: ?)

              ↓ Homomorphic Average
        
    CT_AVG = CT_SUM / 3 (encrypted division)
    CT_AVG 🔐 (average: ?)

              ↓ Threshold Comparison (encrypted!)
        
    CT_THRESHOLD 🔐 (6000, representing 60%)
    CT_DIFF = CT_AVG - CT_THRESHOLD (encrypted!)
    
    Extract sign of CT_DIFF (still encrypted):
    CT_RESULT 🔐 (is average > threshold?)

Output (encrypted):
    CT_RESULT 🔐 (Answer: ?, still secret!)


PHASE 3: DECRYPTION (MPC Threshold Secret Sharing)
══════════════════════════════════════════════════════════════════

                    Key Reconstruction
        
        Decryptor 1 ──┐ (Share 1 of 5)
        Decryptor 2 ──┼─→ [Shamir Interpolation] ──→ Full Key ──→ Decrypt
        Decryptor 3 ──┘ (Share 3 of 5)
        
        Decryptor 4     (offline, not needed)
        Decryptor 5     (offline, not needed)
        
        Requirement: 3-of-5 shares must cooperate
        Security: No single decryptor can decrypt alone

                    Decryption Result
        
    CT_RESULT 🔐 (encrypted) ──→ [Decrypt] ──→ Result (plaintext)
    
    Result: YES
    Reasoning: 63.57% average > 60% threshold
    
                    Sign Result
    
    Each decryptor signs the result:
    sig1 = Sign(Result, key_1)
    sig2 = Sign(Result, key_2)
    sig3 = Sign(Result, key_3)
    
    Multisig = (sig1, sig2, sig3)


PHASE 4: SETTLEMENT (On-Chain Finalization)
══════════════════════════════════════════════════════════════════

    Oracle.finalizeResult(
        eventId,
        result=YES,
        aggregate=6357,
        signatures=[sig1, sig2, sig3]
    )
    
    Smart Contract Verification:
    ✓ Check 3-of-5 signatures valid
    ✓ Verify decryptors authorized
    ✓ Verify aggregate > threshold
    ✓ Replay protection (check nonce)
    
    ↓
    
    Settlement:
    ├─ YES predictors get payout
    ├─ NO predictors lose stake
    ├─ Oracle fee: 2% of total
    ├─ Decryptors fee: 1% each
    └─ Event finalized ✅
```

---

## 3. Component Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    BLOCKSENSE FHE ORACLE                         │
│                      (Component Breakdown)                       │
└──────────────────────────────────────────────────────────────────┘

        ┌─────────────────┐
        │  User Interface │
        │  (Web/Mobile)   │
        └────────┬────────┘
                 │
    ┌────────────▼───────────────┐
    │     SDK LAYER              │
    │   (sdk/encrypt.js)         │
    ├────────────────────────────┤
    │ • quantizePercent()        │
    │ • quantizePrice()          │
    │ • quantizeRatio()          │
    │ • encryptWithZama()        │
    │ • submitEventPrediction()  │
    │ • submitBatchPredictions() │
    └────────────┬───────────────┘
                 │
    ┌────────────▼────────────────┐
    │   BLOCKCHAIN LAYER         │
    │   (Solidity Contract)       │
    ├─────────────────────────────┤
    │ Contract: FHEOracle         │
    │                             │
    │ Functions:                  │
    │ • createEvent()             │
    │ • submitPrediction()        │
    │ • proposeResult()           │
    │ • finalizeResult()          │
    │ • disputeResult()           │
    │ • juryVote()                │
    │                             │
    │ State:                      │
    │ • events[] (all events)     │
    │ • submissions[] (encrypted) │
    │ • disputes[] (challenges)   │
    └────────────┬────────────────┘
                 │
    ┌────────────▼───────────────────────┐
    │   OFFCHAIN COMPUTE LAYER           │
    ├────────────────────────────────────┤
    │                                    │
    │  ┌──────────────────────────────┐ │
    │  │  FHE Engine (Rust)           │ │
    │  │  (examples/zama_integer...rs)│ │
    │  ├──────────────────────────────┤ │
    │  │ • load_ciphertext()          │ │
    │  │ • homomorphic_weighted_sum() │ │
    │  │ • homomorphic_subtract_...() │ │
    │  │ • extract_result()           │ │
    │  │ • aggregate_predictions()    │ │
    │  └──────────────┬───────────────┘ │
    │                 │                  │
    │  ┌──────────────▼────────────────┐ │
    │  │  Decryptor Network            │ │
    │  │  (Distributed Nodes)          │ │
    │  ├───────────────────────────────┤ │
    │  │ • Decryptor 1 (key share 1)   │ │
    │  │ • Decryptor 2 (key share 2)   │ │
    │  │ • Decryptor 3 (key share 3)   │ │
    │  │ • [Decryptor 4 - offline]     │ │
    │  │ • [Decryptor 5 - offline]     │ │
    │  │                               │ │
    │  │ Process:                      │ │
    │  │ 1. Receive CT_RESULT          │ │
    │  │ 2. Sign result locally        │ │
    │  │ 3. Aggregate signatures       │ │
    │  │ 4. Reconstruct full key       │ │
    │  │ 5. Decrypt & finalize         │ │
    │  └───────────────┬───────────────┘ │
    │                 │                  │
    │  ┌──────────────▼────────────────┐ │
    │  │  Result Coordinator           │ │
    │  │  (Aggregator)                 │ │
    │  ├───────────────────────────────┤ │
    │  │ • Collect decryptor shares    │ │
    │  │ • Verify signatures           │ │
    │  │ • Reconstruct key (Lagrange)  │ │
    │  │ • Decrypt final result        │ │
    │  │ • Submit to blockchain        │ │
    │  └───────────────┬───────────────┘ │
    │                 │                  │
    └─────────────────┼──────────────────┘
                      │
                      ▼
        ┌─────────────────────────┐
        │  Oracle.finalizeResult()│
        │  (Settlement on-chain)  │
        └─────────────────────────┘
```

---

## 4. Event State Machine

```
                            ┌─────────────┐
                            │ NOT CREATED │
                            └──────┬──────┘
                                   │
                        CREATE EVENT│
                                   ▼
                            ┌──────────────┐
                     ┌─────→│   PENDING    │◄──────────┐
                     │      │ (Open for    │           │
                     │      │  submissions)│           │
                     │      └──────┬───────┘           │
                     │             │                   │
                     │   AFTER     │                   │
                     │  DEADLINE   │                   │
                     │  (no more   │                   │
                     │ submissions)│                   │
                     │             ▼                   │
                     │    ┌──────────────┐             │
        DISPUTE       │    │  PROPOSED    │             │ WITHDRAW
        PERIOD        │    │ (result ready│             │ (if dispute
        PASSED        │    │  for dispute)│             │  requested)
           │          │    └──────┬───────┘             │
           │          │           │                    │
           │          │  DISPUTE  │                    │
           │          │  RESULT   │                    │
           │          │           ▼                    │
           │          │    ┌──────────────┐            │
           │          │    │  DISPUTED    │            │
           │          │    │ (jury voting)│            │
           │          │    └──────┬───────┘            │
           │          │           │                    │
           │          │  JURY     │                    │
           │          │  VERDICT  │                    │
           │          │           ▼                    │
           │          │    ┌──────────────┐            │
           │          └───→│  RESOLVED    │────────────┘
           │               │ (final result │
           │               │   from jury)  │
           │               └──────────────┘
           │
           ▼
    ┌──────────────┐
    │  FINALIZED   │
    │ (settlement  │
    │  completed)  │
    └──────────────┘
```

---

## 5. Quantization Scales

```
┌──────────────────────────────────────────────────────────┐
│            MARKET TYPES & QUANTIZATION SCALES            │
└──────────────────────────────────────────────────────────┘

PERCENTAGE MARKET
═════════════════════════════════════════════════════════════

Scale: 0 to 10,000
Formula: percentage × 100

Examples:
    0%  ──→  ×   (REJECT - unfalsifiable)
    0.01% ─→  1
    25%  ──→  2,500
    50%  ──→  5,000
    75%  ──→  7,500
    99.99% →  9,999
    100% ──→  ×   (REJECT - unfalsifiable)

Visual:
    0 ├────┼────┼────┼────┼────┼────┼────┼──── 10000
      0%   20%  40%  60%  80%  100%
           │    │    │    │    │
           │    │    │    │    └─ Range for uncertain markets
           │    │    │    └────── Medium confidence
           │    │    └────────────  Range for predictions
           │    └──────────────────  Low confidence
           └──────────────────────── Minimum threshold


PRICE MARKET (USD)
═════════════════════════════════════════════════════════════

Scale: 0 to 10^15 (supports i64 max)
Formula: USD price × 10^8

Examples:
    $0.00000001 (satoshi) ──→      1
    $0.01 (penny) ──────────→  1,000,000
    $1.00 ─────────────────→  100,000,000
    $100.00 ────────────────→  10,000,000,000
    $3,250.50 (ETH price) ──→  325,050,000,000
    $50,000 (BTC-like) ─────→  5,000,000,000,000
    $92,233,720 (i64 max) ──→  9,223,372,000,000,000

Range:
    0 $ ├─ satoshi ─┼─ penny ─┼─ dollar ─┼─ million ─┼─ billion ─┤ $92.2B
        └─ 1  ─┼─ 1M ─┼─ 100M ─┼─ 100M*K ─┼─ 100M*K*K ─┴─ $92.2B


RATIO MARKET (0.0 to 1.0)
═════════════════════════════════════════════════════════════

Scale: 0 to 1,000,000
Formula: ratio × 10^6

Examples:
    0.000001 (0.0001%) ──→      1
    0.01 (1%) ──────────→   10,000
    0.10 (10%) ──────────→  100,000
    0.50 (50%, fair) ────→  500,000
    0.527 (52.7%) ───────→  527,000
    0.99 (99%) ──────────→  990,000
    1.00 (100%, certain)──→  1,000,000

Precision: 0.0001% (10^-6 precision)
```

---

## 6. Security Model

```
┌──────────────────────────────────────────────────┐
│         THREAT vs MITIGATION LAYERS              │
└──────────────────────────────────────────────────┘

THREAT LAYER 1: Information Leakage
═════════════════════════════════════
Attacker Goal: Learn individual predictions

Defenses:
    ├─ FHE Encryption 🔐
    │  └─ Ciphertexts reveal nothing about plaintext
    │  └─ No decryption until final result
    │
    ├─ Timing Attack Prevention ⏱️
    │  └─ Constant-time operations
    │  └─ Noise in computation
    │
    └─ Network Privacy 🌐
       └─ HTTPS for all submissions
       └─ Anonymize IP addresses


THREAT LAYER 2: Decryptor Collusion
═════════════════════════════════════
Attacker Goal: Decrypt result early (< threshold)

Defenses:
    ├─ Threshold Cryptography 🔑
    │  └─ Requires 3 of 5 decryptors
    │  └─ Any 2 are insufficient
    │
    ├─ Shamir's Secret Sharing 📦
    │  └─ Information-theoretic security
    │  └─ Unbreakable without t shares
    │
    └─ Incentive Alignment 💰
       └─ Bonding mechanism for decryptors
       └─ Slashing for dishonest behavior


THREAT LAYER 3: Oracle Manipulation
═════════════════════════════════════
Attacker Goal: Control final result

Defenses:
    ├─ Mathematical Verification ✓
    │  └─ FHE guarantees correct aggregation
    │  └─ No manipulation possible mathematically
    │
    ├─ Dispute Mechanism 🏛️
    │  └─ Anyone can challenge result
    │  └─ Jury arbitration if disputed
    │
    └─ Multisig Requirement ✍️
       └─ 3+ independent signatures required
       └─ No single entity can finalize


THREAT LAYER 4: Smart Contract Bugs
═════════════════════════════════════
Attacker Goal: Exploit code vulnerability

Defenses:
    ├─ Formal Verification 🔬
    │  └─ Mathematical proofs of correctness
    │  └─ Solidity best practices
    │
    ├─ External Audit 👁️
    │  └─ Professional security review
    │  └─ Penetration testing
    │
    ├─ Insurance Pool 🛡️
    │  └─ Coverage for exploits
    │  └─ Gradual rollout (canary deployment)
    │
    └─ Upgradeable Pattern 🔄
       └─ Proxy pattern for fixes
       └─ Community governance


Trust Assumptions Summary:
══════════════════════════════════════

✅ FHE is secure (Zama library correct)
✅ ≥3 of 5 decryptors are honest
✅ Ethereum consensus is secure
✅ Smart contract is correctly audited
✅ Jury members act in good faith

❌ NOT assumed:
   • All decryptors are honest (only 3-of-5)
   • Single entity controls oracle (multisig required)
   • Predictions remain hidden forever (only until finalization)
   • Contract has no bugs (external audit covers known risks)
```

---

## 7. Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│              DEPLOYMENT ENVIRONMENTS                    │
└─────────────────────────────────────────────────────────┘

LOCAL DEVELOPMENT
═════════════════════════════════════════════════════════════

┌──────────────────────────────────────┐
│    Hardhat Local Node                │
│    (In-memory blockchain)            │
├──────────────────────────────────────┤
│                                      │
│  FHEOracle.sol (deployed locally)    │
│  ├─ 20 test accounts (pre-funded)    │
│  ├─ Fast block time (~0s)            │
│  ├─ Unlimited funds                  │
│  └─ Mock Zama (for testing)          │
│                                      │
│  Usage: npm run test                 │
│  URL: http://127.0.0.1:8545          │
│                                      │
└──────────────────────────────────────┘


TESTNET (Sepolia)
═════════════════════════════════════════════════════════════

┌──────────────────────────────────────┐
│    Ethereum Sepolia Network          │
│    (Public testnet, real blockchain) │
├──────────────────────────────────────┤
│                                      │
│  FHEOracle.sol (public contract)     │
│  ├─ Real Zama library                │
│  ├─ Real decryptor network (3+)      │
│  ├─ Block time ~13s                  │
│  ├─ Test ETH from faucet             │
│  └─ Etherscan verification           │
│                                      │
│  Usage: npm run deploy:sepolia       │
│  URL: https://sepolia.etherscan.io   │
│  RPC: https://sepolia.infura.io      │
│                                      │
└──────────────────────────────────────┘


MAINNET (Production)
═════════════════════════════════════════════════════════════

┌────────────────────────────────────────┐
│    Ethereum Mainnet                    │
│    (Production, real value)            │
├────────────────────────────────────────┤
│                                        │
│  FHEOracle.sol (upgraded proxy)        │
│  ├─ Audited Zama library               │
│  ├─ Decryptor network (10+ geog)       │
│  ├─ Block time ~13s                    │
│  ├─ Real user predictions & stakes     │
│  ├─ Insurance pool for edge cases      │
│  └─ Gradual rollout (canary)           │
│                                        │
│  Usage: npm run deploy:mainnet         │
│  URL: https://etherscan.io             │
│  RPC: https://mainnet.infura.io        │
│                                        │
│  Requirements:
│  ✓ Mainnet ETH for gas
│  ✓ Private key secure (HSM recommended)
│  ✓ Team 2-of-3 multisig approval
│  ✓ Insurance policy active
│  ✓ Emergency pause mechanism enabled
│                                        │
└────────────────────────────────────────┘
```

---

**Status**: ✅ Complete (All Flows Documented)  
**Diagrams**: 7 (High-level, Data flow, Components, State machine, Quantization, Security, Deployment)  
**Total Lines**: 600+  
**Last Updated**: November 6, 2025  
**Version**: 1.1

**Coverage**:
- ✅ Week 2 Data Pipeline flow documented
- ✅ Week 3 FHE Aggregation architecture covered
- ✅ Week 4 Threshold Decryption and MPC flows added
- ✅ Week 5+ Smart contract integration placeholders ready
- ⏳ Performance metrics pending (Week 9)

```
