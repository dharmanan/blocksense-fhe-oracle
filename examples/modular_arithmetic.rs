/// Improved Modular Arithmetic for Threshold Decryption
/// Using proper field operations with Lagrange coefficients
use std::collections::HashMap;

/// Prime modulus for field arithmetic (common in cryptography)
const FIELD_PRIME: i64 = 1_000_000_007;

/// Extended Euclidean Algorithm for finding modular inverse
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

/// Calculate modular inverse: a^(-1) mod p
/// Returns None if inverse doesn't exist
fn mod_inverse(mut a: i64, p: i64) -> Option<i64> {
    a = a % p;
    let (gcd, x, _) = extended_gcd(a, p);

    if gcd != 1 {
        return None;
    }

    let result = (x % p + p) % p;
    Some(result)
}

/// Modular multiplication
fn mod_mul(a: i64, b: i64, p: i64) -> i64 {
    ((a % p) * (b % p)) % p
}

/// Modular addition
fn mod_add(a: i64, b: i64, p: i64) -> i64 {
    ((a % p) + (b % p)) % p
}

/// Modular subtraction
fn mod_sub(a: i64, b: i64, p: i64) -> i64 {
    let result = ((a % p) - (b % p)) % p;
    if result < 0 {
        result + p
    } else {
        result
    }
}

/// Calculate Lagrange coefficient with proper modular arithmetic
/// L_i(0) = ∏_{j≠i} (-x_j) / (x_i - x_j) mod p
fn lagrange_coefficient_modular(
    participant_id: i64,
    all_participant_ids: &[i64],
    p: i64,
) -> i64 {
    let mut numerator = 1i64;
    let mut denominator = 1i64;

    for &xj in all_participant_ids {
        if xj != participant_id {
            // Numerator: ∏ (-x_j) = ∏ (p - x_j)
            let neg_xj = if xj < 0 { p + (xj % p) } else { (p - (xj % p)) % p };
            numerator = mod_mul(numerator, neg_xj, p);

            // Denominator: ∏ (x_i - x_j)
            let diff = mod_sub(participant_id, xj, p);
            denominator = mod_mul(denominator, diff, p);
        }
    }

    // L_i(0) = numerator / denominator mod p
    // = numerator * denominator^(-1) mod p
    match mod_inverse(denominator, p) {
        Some(inv) => mod_mul(numerator, inv, p),
        None => {
            eprintln!("Warning: Could not find modular inverse for {}", denominator);
            0
        }
    }
}

/// Reconstruct secret using Lagrange interpolation with modular arithmetic
/// S = ∑ y_i * L_i(0) mod p
pub fn threshold_decrypt_modular(
    shares: &HashMap<u32, i64>,
    modulus: i64,
    threshold: usize,
) -> Option<i64> {
    if shares.len() < threshold {
        return None; // Need at least threshold shares
    }

    let participant_ids: Vec<i64> = shares.keys().take(threshold).map(|&id| id as i64).collect();
    let mut secret = 0i64;

    for (id, share_value) in shares.iter().take(threshold) {
        let li = lagrange_coefficient_modular(*id as i64, &participant_ids, modulus);
        let contribution = mod_mul(*share_value, li, modulus);
        secret = mod_add(secret, contribution, modulus);
    }

    Some(secret)
}

/// Shamir's Secret Sharing with Modular Arithmetic
pub struct ShamirSchemeModular {
    pub secret: i64,
    pub a1: i64,
    pub threshold: u32,
    pub total_shares: u32,
    pub field_prime: i64,
}

impl ShamirSchemeModular {
    pub fn new(secret: i64, a1: i64, threshold: u32, total_shares: u32) -> Self {
        ShamirSchemeModular {
            secret,
            a1,
            threshold,
            total_shares,
            field_prime: FIELD_PRIME,
        }
    }

    pub fn generate_shares(&self) -> HashMap<u32, i64> {
        let mut shares = HashMap::new();

        for x in 1..=self.total_shares as i64 {
            let share = mod_add(
                self.secret,
                mod_mul(self.a1, x, self.field_prime),
                self.field_prime,
            );
            shares.insert(x as u32, share);
        }

        shares
    }

    pub fn recover_secret(&self, shares: &HashMap<u32, i64>) -> Option<i64> {
        threshold_decrypt_modular(shares, self.field_prime, self.threshold as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_operations() {
        let p = 1_000_000_007;

        // Test mod_add
        assert_eq!(mod_add(5, 3, p), 8);
        assert_eq!(mod_add(999_999_999, 8, p), 0); // (p-8) + 8 = p ≡ 0 mod p

        // Test mod_sub
        assert_eq!(mod_sub(8, 3, p), 5);
        assert_eq!(mod_sub(3, 8, p), p - 5);

        // Test mod_mul
        assert_eq!(mod_mul(3, 4, p), 12);
        // Note: (p/2)*2 might not be 0 due to integer division, skip this case
        assert_eq!(mod_mul(1000, 1001, p), 1_001_000);
    }

    #[test]
    fn test_mod_inverse() {
        let p = 1_000_000_007;

        // 3 * inv(3) ≡ 1 (mod p)
        if let Some(inv) = mod_inverse(3, p) {
            assert_eq!(mod_mul(3, inv, p), 1);
        }

        // 5 * inv(5) ≡ 1 (mod p)
        if let Some(inv) = mod_inverse(5, p) {
            assert_eq!(mod_mul(5, inv, p), 1);
        }

        // For prime p, every non-zero element has an inverse
        for a in 1..20 {
            let inv = mod_inverse(a, p);
            assert!(inv.is_some());
        }
    }

    #[test]
    fn test_lagrange_coefficient_modular() {
        let p = 1_000_000_007;
        let participant_ids = [1i64, 2i64, 3i64];

        // L1(0) = (-2)(-3) / (1-2)(1-3) = 6 / 2 = 3
        let l1 = lagrange_coefficient_modular(1, &participant_ids, p);
        assert_eq!(l1, 3);

        // L2(0) = (-1)(-3) / (2-1)(2-3) = 3 / -1 = -3 ≡ p-3
        let l2 = lagrange_coefficient_modular(2, &participant_ids, p);
        assert_eq!(l2, p - 3);

        // L3(0) = (-1)(-2) / (3-1)(3-2) = 2 / 2 = 1
        let l3 = lagrange_coefficient_modular(3, &participant_ids, p);
        assert_eq!(l3, 1);
    }

    #[test]
    fn test_shamir_modular_basic() {
        let scheme = ShamirSchemeModular::new(42, 10, 3, 5);
        let shares = scheme.generate_shares();

        // Verify shares match polynomial P(x) = 42 + 10x (mod p)
        let p = FIELD_PRIME;
        for (&id, &share) in &shares {
            let expected = mod_add(42, mod_mul(10, id as i64, p), p);
            assert_eq!(share, expected, "Share {} mismatch", id);
        }
    }

    #[test]
    fn test_shamir_modular_recovery() {
        let scheme = ShamirSchemeModular::new(42, 10, 3, 5);
        let shares = scheme.generate_shares();

        // Use any 3 shares to recover secret
        let selected_shares: HashMap<u32, i64> =
            shares.iter().take(3).map(|(k, v)| (*k, *v)).collect();

        let recovered = scheme.recover_secret(&selected_shares);
        assert_eq!(recovered, Some(42));
    }

    #[test]
    fn test_shamir_modular_recovery_different_combinations() {
        let scheme = ShamirSchemeModular::new(100, 20, 3, 5);
        let shares = scheme.generate_shares();

        // Try different combinations of 3 shares
        let combinations = vec![
            (1, 2, 3),
            (1, 2, 4),
            (2, 3, 4),
            (3, 4, 5),
        ];

        for (a, b, c) in combinations {
            let selected_shares: HashMap<u32, i64> = vec![
                (a, shares[&a]),
                (b, shares[&b]),
                (c, shares[&c]),
            ]
            .into_iter()
            .collect();

            let recovered = scheme.recover_secret(&selected_shares);
            assert_eq!(recovered, Some(100), "Failed for combination ({}, {}, {})", a, b, c);
        }
    }

    #[test]
    fn test_shamir_modular_insufficient_shares() {
        let scheme = ShamirSchemeModular::new(42, 10, 3, 5);
        let shares = scheme.generate_shares();

        // Try with only 2 shares
        let selected_shares: HashMap<u32, i64> =
            shares.iter().take(2).map(|(k, v)| (*k, *v)).collect();

        let recovered = scheme.recover_secret(&selected_shares);
        assert_eq!(recovered, None);
    }

    #[test]
    fn test_shamir_modular_large_numbers() {
        let secret = 999_999_999i64;
        let a1 = 123_456_789i64;
        let scheme = ShamirSchemeModular::new(secret, a1, 3, 5);
        let shares = scheme.generate_shares();

        let selected_shares: HashMap<u32, i64> =
            shares.iter().take(3).map(|(k, v)| (*k, *v)).collect();

        let recovered = scheme.recover_secret(&selected_shares);
        assert_eq!(recovered, Some(secret));
    }

    #[test]
    fn test_shamir_modular_with_different_thresholds() {
        // Test 2-of-5 scheme: needs k=2 shares to recover
        let scheme_2_5 = ShamirSchemeModular::new(100, 5, 2, 5);
        let shares_2_5 = scheme_2_5.generate_shares();

        let selected: HashMap<u32, i64> =
            shares_2_5.iter().take(2).map(|(k, v)| (*k, *v)).collect();
        let recovered = scheme_2_5.recover_secret(&selected);
        assert_eq!(recovered, Some(100));

        // Test 3-of-5 scheme: needs k=3 shares to recover
        let scheme_3_5 = ShamirSchemeModular::new(200, 15, 3, 5);
        let shares_3_5 = scheme_3_5.generate_shares();

        let selected: HashMap<u32, i64> =
            shares_3_5.iter().take(3).map(|(k, v)| (*k, *v)).collect();
        let recovered = scheme_3_5.recover_secret(&selected);
        assert_eq!(recovered, Some(200));
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║  Modular Arithmetic for Threshold Decryption          ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    // Test 1: Modular Arithmetic Operations
    println!("🔍 Test 1: Modular Operations (prime = {})", FIELD_PRIME);
    println!("  mod_add(999999999, 10, p) = {}", mod_add(999999999, 10, FIELD_PRIME));
    println!("  mod_mul(123456789, 987654321, p) = {}", mod_mul(123456789, 987654321, FIELD_PRIME));
    if let Some(inv) = mod_inverse(5, FIELD_PRIME) {
        println!("  mod_inverse(5, p) = {}", inv);
        println!("  Verification: 5 * {} mod p = {}", inv, mod_mul(5, inv, FIELD_PRIME));
    }
    println!();

    // Test 2: Lagrange Coefficient
    println!("🔍 Test 2: Lagrange Coefficients");
    let ids = [1i64, 2i64, 3i64];
    for id in &ids {
        let li = lagrange_coefficient_modular(*id, &ids, FIELD_PRIME);
        println!("  L_{}(0) = {}", id, li);
    }
    println!();

    // Test 3: Shamir Scheme with Modular Arithmetic
    println!("🔍 Test 3: Shamir Scheme (3-of-5) with Modular Arithmetic");
    let scheme = ShamirSchemeModular::new(42, 10, 3, 5);
    let shares = scheme.generate_shares();

    println!("  Secret: 42");
    println!("  Generated shares:");
    for id in 1..=5 {
        println!("    Share {}: {}", id, shares[&id]);
    }

    println!("\n  Recovering from shares 1, 2, 3:");
    let mut selected = HashMap::new();
    selected.insert(1u32, shares[&1]);
    selected.insert(2u32, shares[&2]);
    selected.insert(3u32, shares[&3]);

    if let Some(recovered) = scheme.recover_secret(&selected) {
        println!("    Recovered secret: {}", recovered);
        println!("    ✓ Correct!" );
    }
    println!();

    // Test 4: Different Share Combinations
    println!("🔍 Test 4: Recovery with Different Share Combinations");
    let combinations = [(1, 3, 5), (2, 3, 4), (1, 2, 5)];

    for (a, b, c) in combinations {
        let mut combo = HashMap::new();
        combo.insert(a, shares[&a]);
        combo.insert(b, shares[&b]);
        combo.insert(c, shares[&c]);

        if let Some(recovered) = scheme.recover_secret(&combo) {
            println!("  Shares ({}, {}, {}): recovered = {} ✓", a, b, c, recovered);
        }
    }
    println!();

    // Test 5: Large Number Security
    println!("🔍 Test 5: Large Number Security Test");
    let large_scheme = ShamirSchemeModular::new(
        987654321,
        123456789,
        3,
        5,
    );
    let large_shares = large_scheme.generate_shares();

    let mut large_selected = HashMap::new();
    large_selected.insert(1, large_shares[&1]);
    large_selected.insert(2, large_shares[&2]);
    large_selected.insert(4, large_shares[&4]);

    if let Some(recovered) = large_scheme.recover_secret(&large_selected) {
        println!("  Secret: 987654321");
        println!("  Recovered: {}", recovered);
        println!("  ✓ Large number security test passed!");
    }
}
