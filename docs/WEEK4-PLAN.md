# Week 4: Threshold Decryption - Implementation Plan

**Status**: IN PROGRESS  
**Date**: November 6, 2025  
**Sprint**: Week 4 of 10-week MVP

---

## Overview

This week focuses on implementing **Shamir's Secret Sharing** for distributed threshold decryption. The oracle result (encrypted comparison) will be decrypted by a committee of 5 decryptors, where any 3 of them can perform the decryption.

---

## Goals

### Primary: Shamir's Secret Sharing (3-of-5)
- [ ] Key share generation
- [ ] Lagrange interpolation
- [ ] Threshold decryption reconstruction
- [ ] Verification of shares

### Secondary: Decryptor Infrastructure
- [ ] Decryptor node implementation
- [ ] Key share distribution protocol
- [ ] Signature aggregation
- [ ] Integration with Week 3 FHE compute

---

## Architecture

### System Diagram

```
┌─────────────────────────────────────────┐
│ Encrypted Result from FHE Oracle        │
│ (Week 3 deliverable)                    │
└──────────────┬──────────────────────────┘
               │
        ┌──────▼──────┐
        │ Threshold   │
        │ Decryption  │
        └──────┬──────┘
               │
    ┌──────────┼──────────┐
    │          │          │
    ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌────────┐
│ Decrypt│ │ Decrypt│ │ Decrypt│
│ Node 1 │ │ Node 2 │ │ Node 3 │
│ Share1 │ │ Share2 │ │ Share3 │
└─────┬──┘ └─────┬──┘ └─────┬──┘
      │          │          │
      └──────────┼──────────┘
                 │
            ┌────▼─────┐
            │ Reconstruct│
            │ Secret    │
            └────┬─────┘
                 │
                 ▼
          Final Oracle Result
          (YES/NO)
```

### Key Components

```
┌──────────────────────────────────┐
│ threshold_decryption.rs          │
├──────────────────────────────────┤
│ • ThresholdConfig                │
│ • SecretShare                    │
│ • Decryptor                      │
│ • ThresholdScheme                │
│ • lagrange_coefficient()         │
│ • threshold_decrypt()            │
│ • generate_key_shares()          │
└──────────────────────────────────┘
```

---

## Implementation Details

### 1. Shamir's Secret Sharing (3-of-5)

**Mathematical Foundation**:
- Secret S is the constant term of polynomial P(x) = S + a₁x + a₂x² + ... + a₂x²
- Evaluate P at points 1, 2, 3, 4, 5 to get shares
- Any 3 shares can reconstruct via Lagrange interpolation

**Share Generation**:
```
Secret: 42 (what we want to share)
Polynomial: P(x) = 42 + 17x + 23x²

Shares:
- Share 1: P(1) = 42 + 17 + 23 = 82
- Share 2: P(2) = 42 + 34 + 92 = 168
- Share 3: P(3) = 42 + 51 + 207 = 300
- Share 4: P(4) = 42 + 68 + 368 = 478
- Share 5: P(5) = 42 + 85 + 575 = 702

Recovery with shares 1, 2, 3:
P(0) = 82 * L₁(0) + 168 * L₂(0) + 300 * L₃(0) = 42 ✓
```

**Lagrange Coefficients**:
```
L_i(x) = ∏(x - x_j)/(x_i - x_j) for j ≠ i

At x=0 (secret point):
L₁(0) = (0-2)(0-3) / (1-2)(1-3) = 6/2 = 3
L₂(0) = (0-1)(0-3) / (2-1)(2-3) = 3/-1 = -3
L₃(0) = (0-1)(0-2) / (3-1)(3-2) = 2/2 = 1
```

### 2. Decryptor Infrastructure

**Decryptor Node Structure**:
```rust
pub struct Decryptor {
    pub id: u32,                    // 1-5
    pub name: String,              // "Decryptor A", etc.
    pub key_share: SecretShare,     // Their share
}
```

**Threshold Scheme**:
```rust
pub struct ThresholdScheme {
    pub config: ThresholdConfig,    // 3-of-5
    pub decryptors: Vec<Decryptor>, // Registered nodes
}
```

### 3. Decryption Process

```
Step 1: Collect shares from 3+ decryptors
Step 2: Compute Lagrange coefficients
Step 3: Reconstruct secret: S = Σ share_i * L_i(0)
Step 4: Apply to encrypted result
Step 5: Verify result matches expected outcome
```

---

## Integration with Week 3

### Input from FHE Compute
```
AggregationResult {
    aggregate_ciphertext: CT_agg,
    threshold_ciphertext: CT_thresh,
    diff_ciphertext: CT_diff,
    comparison_result_ciphertext: CT_result,  // What we decrypt
    metadata: {...}
}
```

### Threshold Decryption Flow
```
1. FHE Oracle computes (encrypted): result = (aggregate > threshold)
2. Result stored in comparison_result_ciphertext
3. Decryptors apply their key shares
4. Reconstruct: plaintext_result = threshold_decrypt(...)
5. Output: Oracle Decision (YES/NO)
```

---

## Test Plan

### Unit Tests
```
✓ ThresholdConfig validation
✓ Decryptor registration
✓ Key share generation
✓ Lagrange coefficient computation
✓ Threshold decryption with exactly k shares
✓ Threshold decryption with k+1 shares
✓ Failure when k-1 shares
✓ Share verification
```

### Integration Tests
```
✓ End-to-end with Week 3 FHE result
✓ Multiple decryptors joining
✓ Different subsets of k decryptors
✓ Deterministic output (same result)
✓ Performance: <1s decryption
```

### Security Tests
```
✓ No information leak from k-1 shares
✓ Share integrity verification
✓ Byzantine resilience (k-1 corrupted shares)
```

---

## Deliverables

### Code
- `examples/threshold_decryption.rs` - Core implementation
- `examples/week4_example.rs` - End-to-end example
- `examples/test_threshold.rs` - Test suite
- `docs/WEEK4-IMPLEMENTATION.md` - Technical details

### Documentation
- Week 4 Completion Report
- Shamir's Secret Sharing explanation
- Integration guide with FHE Oracle

### Testing
- 15+ unit tests (all passing)
- 5+ integration tests
- Security verification

---

## Success Criteria

| Criterion | Target | Status |
|-----------|--------|--------|
| 3-of-5 threshold working | ✓ | Pending |
| Key shares correctly generated | ✓ | Pending |
| Lagrange interpolation correct | ✓ | Pending |
| Decryption deterministic | ✓ | Pending |
| Works with Week 3 FHE result | ✓ | Pending |
| All 15+ tests passing | ✓ | Pending |
| <1s decryption time | ✓ | Pending |
| Comprehensive documentation | ✓ | Pending |

---

## Timeline

**Today (Nov 6)**:
- Setup and planning (✓ done)
- Core implementation
- Basic tests

**Later**:
- Integration tests
- Documentation
- Commit and move to Week 5

---

## References

### Shamir's Secret Sharing
- **Paper**: "How to Share a Secret" by Adi Shamir (1979)
- **Key Insight**: Any k shares can recover secret, k-1 shares reveal nothing
- **Application**: Threshold signatures, multi-sig wallets

### Related Standards
- IETF RFC 3394: AES Key Wrap
- ECDSA Threshold Signatures
- BLS Threshold Signatures

---

## Notes

### Current Implementation Status
- ✓ Data structures defined
- ✓ Configuration validation
- ✓ Placeholder for key generation
- ⏳ Lagrange interpolation (needs proper modular arithmetic)
- ⏳ Threshold decryption function
- ⏳ Integration tests

### TODO This Week
1. Implement proper Lagrange coefficient with modular arithmetic
2. Test with synthetic secrets
3. Integration with FHE result
4. End-to-end example
5. Documentation

---

**Status**: Implementation in progress  
**Next**: Code implementation and testing
