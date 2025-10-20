# Blocksense + Zama (FHE) Integration Guide# Blocksense + Zama (FHE) Integration Guide# Blocksense + Zama (FHE) Integration Guide# Blocksense + Zama (FHE) Entegrasyon — Teknik Özet



This document describes the POC workflow for integrating Blocksense indexer/adapter with Zama's FHE compute layer.



## Architecture OverviewThis document describes the proof-of-concept (POC) workflow for integrating Blocksense indexer/adapter with Zama's FHE (Fully Homomorphic Encryption) compute layer.



### 1. Data Provider → Adapter Layer



- **Normalization**: Convert raw event data to standardized integer format## Architecture OverviewThis document describes the proof-of-concept (POC) workflow for integrating Blocksense indexer/adapter with Zama's FHE (Fully Homomorphic Encryption) compute layer.Bu doküman Blocksense indexer/adapter ile Zama (FHE) compute'u birleştiren POC akışını detaylı anlatır.

- **Quantization**: Transform percentages into discrete integer scale (e.g., 0..10000)

- **Encryption**: Encrypt using Zama public key



### 2. FHE Compute Layer### 1. Data Provider → Adapter Layer



- **Homomorphic Aggregation**: Compute weighted sum on ciphertexts

- **Threshold Comparison**: Evaluate aggregate > threshold on encrypted data

- **Privacy**: No decryption until final result- **Normalization**: Convert raw event data (eventId, timestamp, value) to standardized integer format## Architecture Overview## Veri Akışı



### 3. Threshold Decryption (MPC)- **Quantization**: Transform percentages or continuous values into discrete integer scale (e.g., 0..10000)



- **Multisig**: Distributed key shares (3-of-5 example)- **Encryption**: Encrypt quantized integer using Zama public key

- **Signatures**: Decryptors sign result with proof



### 4. On-Chain Finalization

### 2. FHE Compute Layer### 1. Data Provider → Adapter Layer```

- **Contract**: Submit result + signatures to Solidity oracle

- **Payouts**: Settlement based on finalized result



## Data Flow- **Homomorphic Aggregation**: Compute weighted sum (Σ weight_i * value_i) on ciphertexts[Data Provider] 



```- **Threshold Comparison**: Evaluate aggregate > threshold directly on encrypted data

Provider 1 (60%) ──┐

Provider 2 (55%) ──┼─→ Encrypt ─→ FHE Sum ─→ MPC Decrypt ─→ On-Chain- **No decryption needed** until final threshold decision- **Normalization**: Convert raw event data (eventId, timestamp, value) to standardized integer format    ↓

Provider 3 (70%) ──┘

```



## Key Parameters### 3. Threshold Decryption (MPC)- **Quantization**: Transform percentages or continuous values into discrete integer scale (e.g., 0..10000)[Adapter: Normalize + Quantize] 



- **Quantization**: percentages × 100 = 0..10000

- **Threshold**: market-specific aggregate value

- **MPC**: 3-of-5 key shares minimum- **Multisig/MPC Protocol**: Distributed key shares (e.g., 3-of-5) needed to decrypt result- **Encryption**: Encrypt quantized integer using Zama public key    ↓

- **Dispute Window**: 1 day configurable

- **Signature Generation**: Each decryptor signs the result along with proof-of-computation

## Security

[Encrypt w/ Zama PubKey] 

1. **FHE Leakage**: Add noise for computation privacy

2. **Key Management**: HSM-backed distributed shares### 4. On-Chain Finalization

3. **Decryptors**: Independent, bonded, incentivized

4. **Disputes**: Jury arbitration mechanism### 2. FHE Compute Layer    ↓



## Testing- **Result Submission**: resultHash + signed receipts sent to Solidity contract



- Unit tests: quantization logic- **ZK Proof (optional)**: Include zero-knowledge proof of correct computation[FHE Compute: Homomorfik Sum] 

- Integration tests: mock Zama library

- E2E tests: 5 sample events on testnet- **Settlement**: Contract distributes payouts based on finalized result

- Performance: latency and cost benchmarks

- **Homomorphic Aggregation**: Compute weighted sum (Σ weight_i * value_i) on ciphertexts    ↓

See `docs/mvp-sprint-plan.md` for detailed roadmap.

## Data Flow Diagram

- **Threshold Comparison**: Evaluate aggregate > threshold directly on encrypted data[Threshold Decrypt (MPC)] 

```

Provider 1 (60%) → Quantize (6000) → Encrypt → CT_1- **No decryption needed** until final threshold decision    ↓

Provider 2 (55%) → Quantize (5500) → Encrypt → CT_2

Provider 3 (70%) → Quantize (7000) → Encrypt → CT_3[On-chain Finalize]

                        ↓

                    FHE Aggregate### 3. Threshold Decryption (MPC)```

                   (on ciphertexts)

                        ↓

                  Threshold Compare

                   (CT_diff > 0?)- **Multisig/MPC Protocol**: Distributed key shares (e.g., 3-of-5) needed to decrypt result## Adımlar

                        ↓

                  Decrypt (3/5 MPC)- **Signature Generation**: Each decryptor signs the result along with proof-of-computation

                        ↓

                  Finalize on-chain### 1. Data Provider → Adapter

```

### 4. On-Chain Finalization

## Key Parameters

Veri kaynağı (API, on-chain event, etc.) → Normalized data:

- **Quantization Base**: 100 (percentages) or 1000 (price feeds)

- **Threshold**: Market-specific (e.g., aggregate > 5500)- **Result Submission**: resultHash + signed receipts sent to Solidity contract

- **MPC Threshold**: 3-of-5 key shares

- **Dispute Window**: 1 day (configurable per market)- **ZK Proof (optional)**: Include zero-knowledge proof of correct computation```json



## Security Considerations- **Settlement**: Contract distributes payouts based on finalized result{



1. **FHE Leakage**: Computation transcript may leak information; use noise addition for privacy  "eventId": "0xabc123...",

2. **Key Management**: Private shares must be distributed and stored securely (HSM recommended)

3. **Decryptor Selection**: Ensure decryptors are independent and incentivized to report truthfully## Data Flow Diagram  "providerId": "provider_a",

4. **Dispute Mechanism**: Off-chain jury or on-chain arbitrage to challenge incorrect results

  "rawValue": 60.5,

## Testing Strategy

```  "timestamp": 1697000000,

- Unit tests for quantization logic

- Integration tests with mock Zama libraryProvider 1 (60%) → Quantize (6000) → Encrypt → CT_1  "unit": "percent"

- End-to-end tests on testnet with 3-5 sample events

- Performance benchmarks (latency, compute cost)Provider 2 (55%) → Quantize (5500) → Encrypt → CT_2}



For detailed sprint roadmap, see `docs/mvp-sprint-plan.md`.Provider 3 (70%) → Quantize (7000) → Encrypt → CT_3```


                        ↓

                    FHE Aggregate**Quant & Encode:**

                   (on ciphertexts)- Percent (0-100) → Integer (0-10000): `value_int = floor(rawValue * 100)`

                        ↓- Timestamp sanitize (5-min bucket, etc.)

                  Threshold Compare- Provider weight (0-1000): `weight_int = floor(weight * 100)`

                   (CT_diff > 0?)

                        ↓Örnek (`sdk/encrypt.js`):

                  Decrypt (3/5 MPC)```javascript

                        ↓function quantizePercent(p) {

                  Finalize on-chain  return Math.round(p * 100); // 60.5% → 6050

```}

```

## Key Parameters

### 2. Encryption

- **Quantization Base**: 100 (percentages) or 1000 (price feeds)

- **Threshold**: Market-specific (e.g., aggregate > 5500)**Zama Public Key ile Encrypt:**

- **MPC Threshold**: 3-of-5 key shares

- **Dispute Window**: 1 day (configurable per market)```

Plaintext: value_int (e.g., 6050)

## Security ConsiderationsPublic Key: Zama parameters (from ZAMA_PUB_KEY secret)

Ciphertext: CT_i = Encrypt(value_int, pubkey)

1. **FHE Leakage**: Computation transcript may leak information; use noise addition for privacy```

2. **Key Management**: Private shares must be distributed and stored securely (HSM recommended)

3. **Decryptor Selection**: Ensure decryptors are independent and incentivized to report truthfullyAdapter `CT_i` iletir. Private key hiçbir yerde yoktur (threshold decrypt sürecinde açılır).

4. **Dispute Mechanism**: Off-chain jury or on-chain arbitrage to challenge incorrect results

### 3. FHE Compute — Homomorfik Toplama

## Testing Strategy

**Server-side (compute node):**

- Unit tests for quantization logic

- Integration tests with mock Zama library```

- End-to-end tests on testnet with 3-5 sample eventsCT_sum = CT_1 * weight_1 + CT_2 * weight_2 + ... + CT_n * weight_n

- Performance benchmarks (latency, compute cost)       = Encrypt(sum(weight_i * value_i))

```

For detailed sprint roadmap, see `docs/mvp-sprint-plan.md`.

Homomorfik özellikle ciphertext'ler üzerinde hesaplama yapılır. Hiç plaintext ile uğraşılmaz.

**Eşik Kontrolü:**

```
diff = CT_sum - threshold_CT
      = Encrypt(sum_weighted - threshold)
if diff > 0: result = YES else result = NO
```

Ciphertext üzerinde karşılaştırma (FHE compare işlemi) veya threshold decrypt edip plaintext'te check.

### 4. Threshold Decryption

**MPC / Multisig Açma (örn. 3-of-5):**

- Key shard'ları 5 decryptor node'a dağıtılmış
- Sonuç, 3+ imza ile açılır
- Hangi 3 açtığı on-chain log'lanır

```
plaintext_result = decrypt_threshold(CT_sum, 3_of_5_shares, pubkey)
                 = Decrypt(CT_sum, partial_keys)
```

**Çıktı:**
```json
{
  "eventId": "0xabc123...",
  "aggregateValue": 6200,  // plaintext
  "result": true,          // aggregate > threshold?
  "decryptorSet": [addr1, addr2, addr3],
  "signature": "0x...",
  "proof": "0x..."
}
```

### 5. On-Chain Finalize

**Solidity (`contracts/FHEOracle.sol`):**

```solidity
finalizeResult(
  eventId,
  resultHash,
  decryptorSet,
  aggregateValue,
  proof
)
```

- resultHash üst seviyede check (e.g., `keccak256(abi.encode(aggregateValue, ...))`)
- decryptorSet doğrulanır (whitelist)
- Dispute window açılır (1 gün)
- Sonra ödeme dağıtılır

## Parametreler & Konfigürasyon

### Zama Setup

```
SECURITY_LEVEL: 128 bit (typical)
MESSAGE_BITS: 32 (integer values up to ~4B)
CARRY_BITS: 4
FRACTIONAL_BITS: 0 (only integers)
```

### Quantization Scale

| Veri Türü | Range | Scale |
|-----------|-------|-------|
| Percent | 0-100 | ×100 → 0-10000 |
| Price | $1-$1M | ×1000 → fixed point |
| Probability | 0-1 | ×1000000 → 0-1M |

### Threshold Config

```json
{
  "threshold": 6500,           // aggregateValue > 6500 → YES
  "min_providers": 3,          // need at least 3 values
  "weights": {
    "provider_a": 100,
    "provider_b": 100,
    "provider_c": 50
  },
  "mpc_k": 3,
  "mpc_n": 5,
  "dispute_window_s": 86400    // 1 day
}
```

## Güvenlik Notları

- **Private Key**: Hiç merkezi olarak saklanmaz. Sadece threshold decrypt share'leri vardır.
- **Ciphertext Integrity**: Zama scheme'i semantik güvenlik sağlar.
- **Dispute Mekanizması**: Yanlış sonuç önerilirse jury challenge edebilir.
- **MPC Openness**: Hangi decryptor'lar açtığı on-chain kaydedilir (accountability).

## Test Senaryoları

### Senaryo 1: Happy Path (3 provider)
```
Provider A: 60% → 6000 (weight 100)
Provider B: 62% → 6200 (weight 100)
Provider C: 58% → 5800 (weight 100)

Aggregate = (6000 + 6200 + 5800) / 3 = 6000
Threshold = 5500
Result = YES (6000 > 5500)
```

### Senaryo 2: Dispute (outlier)
```
Provider A: 60% → 6000 (weight 100)
Provider B: 90% → 9000 (weight 100)  ← anomali
Provider C: 58% → 5800 (weight 100)

Aggregate = (6000 + 9000 + 5800) / 3 = 6933
Result = YES (6933 > 5500)

Jury bilirkişi: Provider B şüpheli → challenge
On-chain dispute flow başlar.
```

### Senaryo 3: Decryption Failure
```
Sadece 2 decryptor'dan imza geliyor (3 gerekli)
On-chain: revert ("insufficient decryption shares")
```

## Kaynaklar

- Zama Concrete: https://github.com/zama-ai/concrete
- FHE Overview: https://en.wikipedia.org/wiki/Homomorphic_encryption
- Threshold Cryptography: https://en.wikipedia.org/wiki/Threshold_cryptosystem
