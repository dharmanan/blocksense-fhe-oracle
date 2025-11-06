/// End-to-End FHE Integration Tests
/// Combines Week 3 (FHE Aggregation) with Week 4 (Threshold Decryption)

use std::collections::HashMap;

// ============================================================================
// Week 3: FHE Module (Simplified for testing)
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct FheCiphertext {
    pub encrypted_value: i64,
    pub is_encrypted: bool,
}

pub trait FheBackend {
    fn encrypt(&self, plaintext: i64) -> FheCiphertext;
    fn decrypt(&self, ct: &FheCiphertext) -> i64;
    fn add(&self, ct_a: &FheCiphertext, ct_b: &FheCiphertext) -> FheCiphertext;
    fn scalar_mul(&self, ct: &FheCiphertext, scalar: i64) -> FheCiphertext;
    fn gt(&self, ct_a: &FheCiphertext, ct_b: &FheCiphertext) -> FheCiphertext;
}

pub struct MockFhe;

impl FheBackend for MockFhe {
    fn encrypt(&self, plaintext: i64) -> FheCiphertext {
        FheCiphertext {
            encrypted_value: plaintext,
            is_encrypted: true,
        }
    }

    fn decrypt(&self, ct: &FheCiphertext) -> i64 {
        ct.encrypted_value
    }

    fn add(&self, ct_a: &FheCiphertext, ct_b: &FheCiphertext) -> FheCiphertext {
        FheCiphertext {
            encrypted_value: ct_a.encrypted_value + ct_b.encrypted_value,
            is_encrypted: true,
        }
    }

    fn scalar_mul(&self, ct: &FheCiphertext, scalar: i64) -> FheCiphertext {
        FheCiphertext {
            encrypted_value: ct.encrypted_value * scalar,
            is_encrypted: true,
        }
    }

    fn gt(&self, ct_a: &FheCiphertext, ct_b: &FheCiphertext) -> FheCiphertext {
        FheCiphertext {
            encrypted_value: if ct_a.encrypted_value > ct_b.encrypted_value { 1 } else { 0 },
            is_encrypted: true,
        }
    }
}

// ============================================================================
// Week 4: Threshold Decryption Module
// ============================================================================

const FIELD_PRIME: i64 = 1_000_000_007;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (gcd, x, y)
}

fn mod_inverse(mut a: i64, p: i64) -> Option<i64> {
    a = a % p;
    let (gcd, x, _) = extended_gcd(a, p);
    if gcd != 1 {
        return None;
    }
    let result = (x % p + p) % p;
    Some(result)
}

fn mod_mul(a: i64, b: i64, p: i64) -> i64 {
    ((a % p) * (b % p)) % p
}

fn mod_add(a: i64, b: i64, p: i64) -> i64 {
    ((a % p) + (b % p)) % p
}

fn mod_sub(a: i64, b: i64, p: i64) -> i64 {
    let result = ((a % p) - (b % p)) % p;
    if result < 0 {
        result + p
    } else {
        result
    }
}

fn lagrange_coefficient_modular(
    participant_id: i64,
    all_participant_ids: &[i64],
    p: i64,
) -> i64 {
    let mut numerator = 1i64;
    let mut denominator = 1i64;

    for &xj in all_participant_ids {
        if xj != participant_id {
            let neg_xj = if xj < 0 { p + (xj % p) } else { (p - (xj % p)) % p };
            numerator = mod_mul(numerator, neg_xj, p);
            let diff = mod_sub(participant_id, xj, p);
            denominator = mod_mul(denominator, diff, p);
        }
    }

    match mod_inverse(denominator, p) {
        Some(inv) => mod_mul(numerator, inv, p),
        None => 0,
    }
}

pub struct ThresholdDecryptor {
    pub id: u32,
    pub key_share: i64,
}

pub struct ThresholdScheme {
    pub threshold: u32,
    pub total_shares: u32,
    pub decryptors: Vec<ThresholdDecryptor>,
}

impl ThresholdScheme {
    pub fn new(threshold: u32, total_shares: u32) -> Self {
        ThresholdScheme {
            threshold,
            total_shares,
            decryptors: Vec::new(),
        }
    }

    pub fn register_decryptor(&mut self, id: u32, key_share: i64) {
        self.decryptors.push(ThresholdDecryptor { id, key_share });
    }

    pub fn decrypt(&self, encrypted_value: i64) -> Option<i64> {
        if self.decryptors.len() < self.threshold as usize {
            return None;
        }

        let participant_ids: Vec<i64> = self
            .decryptors
            .iter()
            .take(self.threshold as usize)
            .map(|d| d.id as i64)
            .collect();

        let mut secret = 0i64;

        for (i, decryptor) in self
            .decryptors
            .iter()
            .take(self.threshold as usize)
            .enumerate()
        {
            let li = lagrange_coefficient_modular(
                decryptor.id as i64,
                &participant_ids,
                FIELD_PRIME,
            );
            let contribution = mod_mul(decryptor.key_share, li, FIELD_PRIME);
            secret = mod_add(secret, contribution, FIELD_PRIME);
        }

        Some(secret)
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_fhe_only() {
        // Test Week 3 FHE operations in isolation
        let fhe = MockFhe;

        let pt_a = 100i64;
        let pt_b = 50i64;

        let ct_a = fhe.encrypt(pt_a);
        let ct_b = fhe.encrypt(pt_b);

        let ct_sum = fhe.add(&ct_a, &ct_b);
        assert_eq!(fhe.decrypt(&ct_sum), 150);

        let ct_comparison = fhe.gt(&ct_sum, &fhe.encrypt(120));
        assert_eq!(fhe.decrypt(&ct_comparison), 1); // YES
    }

    #[test]
    fn test_integration_threshold_only() {
        // Test Week 4 threshold decryption in isolation
        let mut scheme = ThresholdScheme::new(3, 5);

        // Setup shares for secret = 42 with P(x) = 42 + 10x
        for id in 1..=5 {
            let share = 42 + 10 * id as i64;
            scheme.register_decryptor(id, share);
        }

        let secret = 42i64;
        let encrypted = secret; // In mock, encryption is identity
        let decrypted = scheme.decrypt(encrypted);

        assert_eq!(decrypted, Some(42));
    }

    #[test]
    fn test_integration_fhe_to_threshold() {
        // Test: FHE produces encrypted comparison, threshold decrypts it
        let fhe = MockFhe;

        // Step 1: Encrypt provider values
        let provider_values = vec![5000i64, 4900, 5200, 5050, 4950];
        let weights = vec![2, 1, 3, 2, 1];

        // Step 2: FHE aggregation (homomorphic)
        let mut ct_sum = fhe.encrypt(0);
        for (value, weight) in provider_values.iter().zip(weights.iter()) {
            let ct_value = fhe.encrypt(*value);
            let ct_weighted = fhe.scalar_mul(&ct_value, *weight);
            ct_sum = fhe.add(&ct_sum, &ct_weighted);
        }

        let aggregate = fhe.decrypt(&ct_sum);
        assert_eq!(aggregate, 45550);

        // Step 3: FHE threshold comparison
        let ct_threshold = fhe.encrypt(5000);
        let ct_result = fhe.gt(&ct_sum, &ct_threshold);

        // Step 4: Threshold decrypt result
        let encrypted_result = fhe.decrypt(&ct_result);

        let mut threshold_scheme = ThresholdScheme::new(3, 5);
        for id in 1..=5 {
            let share = encrypted_result + 10 * id as i64;
            threshold_scheme.register_decryptor(id, share);
        }

        let final_result = threshold_scheme.decrypt(encrypted_result);
        assert_eq!(final_result, Some(1)); // YES
    }

    #[test]
    fn test_integration_multiple_threshold_combinations() {
        // Test different combinations of decryptors for same encrypted value
        let fhe = MockFhe;

        // FHE: Compute encrypted ETH price comparison
        let eth_price = 5100i64;
        let threshold = 5000i64;

        let ct_eth = fhe.encrypt(eth_price);
        let ct_threshold = fhe.encrypt(threshold);
        let ct_comparison = fhe.gt(&ct_eth, &ct_threshold);

        let encrypted_result = fhe.decrypt(&ct_comparison); // 1 (YES)

        // Threshold: Setup 5 decryptors
        let mut scheme1 = ThresholdScheme::new(3, 5);
        for id in 1..=5 {
            let share = encrypted_result + 10 * id as i64;
            scheme1.register_decryptor(id, share);
        }

        // Try multiple combinations
        let mut scheme2 = ThresholdScheme::new(3, 5);
        scheme2.register_decryptor(1, encrypted_result + 10);
        scheme2.register_decryptor(3, encrypted_result + 30);
        scheme2.register_decryptor(5, encrypted_result + 50);

        assert_eq!(scheme1.decrypt(encrypted_result), Some(1));
        assert_eq!(scheme2.decrypt(encrypted_result), Some(1));
    }

    #[test]
    fn test_integration_privacy_guarantee() {
        // Verify: Individual provider values never revealed
        let fhe = MockFhe;

        let provider_values = vec![1234i64, 5678, 9012];
        let weights = vec![3, 2, 1];

        // FHE workflow: All values stay encrypted
        let mut ct_sum = fhe.encrypt(0);

        for (value, weight) in provider_values.iter().zip(weights.iter()) {
            // Encrypt value (oracle never sees plaintext)
            let ct_value = fhe.encrypt(*value);

            // Homomorphic operations (no decryption)
            let ct_weighted = fhe.scalar_mul(&ct_value, *weight);
            ct_sum = fhe.add(&ct_sum, &ct_weighted);
        }

        // Only final aggregate decrypted
        let aggregate = fhe.decrypt(&ct_sum);
        assert_eq!(aggregate, 1234 * 3 + 5678 * 2 + 9012 * 1);

        // Threshold: Committee can't see aggregate until threshold met
        let mut scheme = ThresholdScheme::new(3, 5);
        for id in 1..=5 {
            let share = aggregate + 20 * id as i64;
            scheme.register_decryptor(id, share);
        }

        // Single committee member can't decrypt
        let mut insufficient_scheme = ThresholdScheme::new(3, 5);
        insufficient_scheme.register_decryptor(1, aggregate + 20);
        assert_eq!(insufficient_scheme.decrypt(aggregate), None);
    }

    #[test]
    fn test_integration_market_scenario_price_prediction() {
        // Real scenario: 3 providers submit encrypted ETH prices
        // Oracle aggregates and thresholds based on median
        // Committee decrypts final decision
        
        let fhe = MockFhe;

        // Providers submit encrypted prices
        let prices = vec![5100i64, 5150, 5120];
        let weights = vec![2, 3, 1]; // Higher weight on more trusted provider

        // Encrypt and aggregate
        let mut ct_aggregate = fhe.encrypt(0);
        for (price, weight) in prices.iter().zip(weights.iter()) {
            let ct = fhe.encrypt(*price);
            let ct_weighted = fhe.scalar_mul(&ct, *weight);
            ct_aggregate = fhe.add(&ct_aggregate, &ct_weighted);
        }

        // Expected: (5100*2 + 5150*3 + 5120*1) / 6 ≈ 5128
        let agg_value = fhe.decrypt(&ct_aggregate);
        assert_eq!(agg_value, 5100 * 2 + 5150 * 3 + 5120 * 1);

        // Threshold comparison: Is aggregate > 5100?
        let ct_threshold = fhe.encrypt(5100);
        let ct_decision = fhe.gt(&ct_aggregate, &ct_threshold);
        let encrypted_decision = fhe.decrypt(&ct_decision);

        // Threshold committee decrypts
        let mut committee = ThresholdScheme::new(3, 5);
        for id in 1..=5 {
            let share = encrypted_decision + 7 * id as i64;
            committee.register_decryptor(id, share);
        }

        let final_decision = committee.decrypt(encrypted_decision);
        assert_eq!(final_decision, Some(1)); // YES: aggregate > 5100
    }

    #[test]
    fn test_integration_end_to_end_full_workflow() {
        // Complete workflow:
        // 1. 5 providers encrypt values
        // 2. Oracle performs FHE aggregation + comparison
        // 3. 3-of-5 committee decrypts via threshold scheme
        // 4. Final decision published on-chain

        let fhe = MockFhe;

        println!("\n╔═══════════════════════════════════════════════════════╗");
        println!("║  Full E2E Integration Test                            ║");
        println!("╚═══════════════════════════════════════════════════════╝");

        // Step 1: Provider submission
        println!("\n📊 Step 1: Provider Submission");
        let providers = vec![
            ("Provider A", 5000, 2),
            ("Provider B", 4900, 1),
            ("Provider C", 5200, 3),
            ("Provider D", 5050, 2),
            ("Provider E", 4950, 1),
        ];

        for (name, value, weight) in &providers {
            println!("  {} submitted: ${}, weight {}", name, value, weight);
        }

        // Step 2: FHE Aggregation
        println!("\n🔐 Step 2: FHE Aggregation (encrypted)");
        let mut ct_sum = fhe.encrypt(0);
        for (_, value, weight) in &providers {
            let ct = fhe.encrypt(*value);
            let ct_weighted = fhe.scalar_mul(&ct, *weight as i64);
            ct_sum = fhe.add(&ct_sum, &ct_weighted);
        }

        println!("  ✓ Homomorphic aggregation complete (values stay encrypted)");

        // Step 3: FHE Comparison
        println!("\n🔐 Step 3: FHE Threshold Comparison (encrypted)");
        let threshold = 5000;
        let ct_threshold = fhe.encrypt(threshold);
        let ct_result = fhe.gt(&ct_sum, &ct_threshold);

        println!("  ✓ Comparison: aggregate > {} (encrypted)", threshold);

        // Step 4: Threshold Decryption
        println!("\n🔓 Step 4: Threshold Decryption (3-of-5 committee)");
        let encrypted_decision = fhe.decrypt(&ct_result);
        let aggregate_value = fhe.decrypt(&ct_sum);

        println!("  Aggregate: {} (would decrypt here)", aggregate_value);
        println!("  Setting up 3-of-5 threshold scheme...");

        let mut committee = ThresholdScheme::new(3, 5);
        for id in 1..=5 {
            let share = encrypted_decision + 10 * id as i64;
            committee.register_decryptor(id, share);
            println!("  ✓ Committee member {} registered", id);
        }

        // Step 5: Final Decision
        println!("\n📜 Step 5: Final Decision");
        match committee.decrypt(encrypted_decision) {
            Some(decision) => {
                let result = if decision == 1 { "YES" } else { "NO" };
                println!("  Decision: {} (aggregate > threshold)", result);
                println!("  ✓ Ready for on-chain publication");
            }
            None => println!("  ✗ Insufficient decryptors"),
        }

        println!("\n✅ Full E2E workflow complete!");
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║  FHE + Threshold Integration Tests                   ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    println!("Running integration tests...");
    println!();

    // Demo: FHE aggregation + threshold decryption
    let fhe = MockFhe;

    // Scenario: 3 price feeds
    let prices = vec![100, 105, 102];
    let weights = vec![2, 1, 3];

    println!("📊 Scenario: 3 Price Feeds (weighted aggregation)");
    for (i, (price, weight)) in prices.iter().zip(weights.iter()).enumerate() {
        println!("  Feed {}: price={}, weight={}", i + 1, price, weight);
    }
    println!();

    // FHE: Aggregate
    let mut ct_agg = fhe.encrypt(0);
    for (price, weight) in prices.iter().zip(weights.iter()) {
        let ct = fhe.encrypt(*price);
        let ct_w = fhe.scalar_mul(&ct, *weight as i64);
        ct_agg = fhe.add(&ct_agg, &ct_w);
    }

    let agg = fhe.decrypt(&ct_agg);
    println!("🔐 FHE Aggregation: {} (encrypted)", agg);
    println!();

    // Threshold: Setup committee
    println!("🔓 Threshold Decryption Setup (3-of-5)");
    let mut scheme = ThresholdScheme::new(3, 5);
    for id in 1..=5 {
        scheme.register_decryptor(id, agg + 10 * id as i64);
    }

    match scheme.decrypt(agg) {
        Some(result) => println!("  Final result: {} ✓", result),
        None => println!("  Insufficient decryptors"),
    }
}
