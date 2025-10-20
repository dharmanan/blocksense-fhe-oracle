# Blocksense + Zama (FHE) Integration Guide

This document describes the POC workflow for integrating Blocksense indexer/adapter with Zama's FHE compute layer.

## Architecture Overview

### 1. Data Provider → Adapter Layer

- **Normalization**: Convert raw event data to standardized integer format
- **Quantization**: Transform percentages into discrete integer scale (e.g., 0..10000)
- **Encryption**: Encrypt using Zama public key

### 2. FHE Compute Layer

- **Homomorphic Aggregation**: Compute weighted sum on ciphertexts
- **Threshold Comparison**: Evaluate aggregate > threshold on encrypted data
- **Privacy**: No decryption until final result

### 3. Threshold Decryption (MPC)

- **Multisig**: Distributed key shares (3-of-5 example)
- **Signatures**: Decryptors sign result with proof

### 4. On-Chain Finalization

- **Contract**: Submit result + signatures to Solidity oracle
- **Payouts**: Settlement based on finalized result

## Data Flow

```
Provider 1 (60%) ──┐
Provider 2 (55%) ──┼─→ Encrypt ─→ FHE Sum ─→ MPC Decrypt ─→ On-Chain
Provider 3 (70%) ──┘
```

## Key Parameters

- **Quantization**: percentages × 100 = 0..10000
- **Threshold**: market-specific aggregate value
- **MPC**: 3-of-5 key shares minimum
- **Dispute Window**: 1 day configurable

## Security

1. **FHE Leakage**: Add noise for computation privacy
2. **Key Management**: HSM-backed distributed shares
3. **Decryptors**: Independent, bonded, incentivized
4. **Disputes**: Jury arbitration mechanism

## Testing

- Unit tests: quantization logic
- Integration tests: mock Zama library
- E2E tests: 5 sample events on testnet
- Performance: latency and cost benchmarks

See `docs/mvp-sprint-plan.md` for detailed roadmap.
