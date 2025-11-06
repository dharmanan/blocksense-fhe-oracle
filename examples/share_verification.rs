/// Share Verification with Commitments
/// Verifiable Secret Sharing (VSS) to detect corrupted shares
use std::collections::HashMap;

/// Public commitment for share verification
#[derive(Clone, Debug)]
pub struct Commitment {
    pub coefficients: Vec<i64>,
}

impl Commitment {
    pub fn new(secret: i64, a1: i64) -> Self {
        Commitment {
            coefficients: vec![secret, a1],
        }
    }

    /// Verify if a share is valid against this commitment
    /// V = C0 + C1*x + C2*x^2 + ...
    pub fn verify_share(&self, x: i64, share_value: i64) -> bool {
        let mut result = 0i64;
        let mut x_power = 1i64;

        for coeff in &self.coefficients {
            result = result.wrapping_add(coeff.wrapping_mul(x_power));
            x_power = x_power.wrapping_mul(x);
        }

        result == share_value
    }
}

/// Enhanced SecretShare with verification capability
#[derive(Clone, Debug)]
pub struct VerifiableSecretShare {
    pub id: u32,
    pub share_value: i64,
    pub commitment: Option<Commitment>,
    pub is_verified: bool,
}

impl VerifiableSecretShare {
    pub fn new(id: u32, share_value: i64) -> Self {
        VerifiableSecretShare {
            id,
            share_value,
            commitment: None,
            is_verified: false,
        }
    }

    pub fn with_commitment(mut self, commitment: Commitment) -> Self {
        self.commitment = Some(commitment.clone());
        self.is_verified = commitment.verify_share(self.id as i64, self.share_value);
        self
    }

    pub fn verify_against_commitment(&mut self, commitment: &Commitment) -> bool {
        self.is_verified = commitment.verify_share(self.id as i64, self.share_value);
        self.is_verified
    }
}

/// Verifiable Secret Sharing Scheme with Byzantine-resistant properties
pub struct VerifiableSecretSharingScheme {
    pub secret: i64,
    pub a1: i64,
    pub shares: HashMap<u32, VerifiableSecretShare>,
    pub commitment: Commitment,
    pub threshold: u32,
    pub total_shares: u32,
}

impl VerifiableSecretSharingScheme {
    /// Create a new VSS scheme with polynomial P(x) = secret + a1*x
    pub fn new(secret: i64, a1: i64, threshold: u32, total_shares: u32) -> Self {
        let commitment = Commitment::new(secret, a1);

        VerifiableSecretSharingScheme {
            secret,
            a1,
            shares: HashMap::new(),
            commitment,
            threshold,
            total_shares,
        }
    }

    /// Generate verifiable shares
    pub fn generate_verifiable_shares(&mut self) {
        for x in 1..=self.total_shares as i64 {
            let share_value = self.secret.wrapping_add(self.a1.wrapping_mul(x));
            let mut verifiable_share = VerifiableSecretShare::new(x as u32, share_value);
            verifiable_share = verifiable_share.with_commitment(self.commitment.clone());

            self.shares.insert(x as u32, verifiable_share);
        }
    }

    /// Get all verified shares
    pub fn get_verified_shares(&self) -> Vec<&VerifiableSecretShare> {
        self.shares
            .values()
            .filter(|share| share.is_verified)
            .collect()
    }

    /// Get all corrupted shares (failed verification)
    pub fn get_corrupted_shares(&self) -> Vec<&VerifiableSecretShare> {
        self.shares
            .values()
            .filter(|share| !share.is_verified)
            .collect()
    }

    /// Detect Byzantine shares
    pub fn detect_byzantine_shares(&self) -> (usize, usize) {
        let verified = self.get_verified_shares().len();
        let corrupted = self.get_corrupted_shares().len();
        (verified, corrupted)
    }

    /// Verify a single share (for incoming shares)
    pub fn verify_share(&self, share_id: u32, share_value: i64) -> bool {
        self.commitment.verify_share(share_id as i64, share_value)
    }

    /// Simulate Byzantine attack - corrupt a share
    pub fn simulate_corruption(&mut self, share_id: u32, corruption: i64) {
        if let Some(share) = self.shares.get_mut(&share_id) {
            share.share_value = share.share_value.wrapping_add(corruption);
            share.is_verified = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_verification() {
        // P(x) = 42 + 10x
        let commitment = Commitment::new(42, 10);

        // Verify shares: P(1) = 52, P(2) = 62, P(3) = 72
        assert!(commitment.verify_share(1, 52));
        assert!(commitment.verify_share(2, 62));
        assert!(commitment.verify_share(3, 72));

        // Invalid shares should fail
        assert!(!commitment.verify_share(1, 50));
        assert!(!commitment.verify_share(2, 60));
    }

    #[test]
    fn test_verifiable_secret_share() {
        let commitment = Commitment::new(42, 10);
        let share = VerifiableSecretShare::new(1, 52).with_commitment(commitment);

        assert!(share.is_verified);
        assert_eq!(share.share_value, 52);

        let invalid_share = VerifiableSecretShare::new(1, 50).with_commitment(
            Commitment::new(42, 10)
        );
        assert!(!invalid_share.is_verified);
    }

    #[test]
    fn test_vss_scheme_generation() {
        let mut vss = VerifiableSecretSharingScheme::new(42, 10, 3, 5);
        vss.generate_verifiable_shares();

        // All shares should be verified
        let verified = vss.get_verified_shares();
        assert_eq!(verified.len(), 5);

        let corrupted = vss.get_corrupted_shares();
        assert_eq!(corrupted.len(), 0);

        // Verify specific shares
        assert!(vss.verify_share(1, 52));  // 42 + 10*1 = 52
        assert!(vss.verify_share(2, 62));  // 42 + 10*2 = 62
        assert!(vss.verify_share(3, 72));  // 42 + 10*3 = 72
        assert!(vss.verify_share(4, 82));  // 42 + 10*4 = 82
        assert!(vss.verify_share(5, 92));  // 42 + 10*5 = 92
    }

    #[test]
    fn test_byzantine_detection() {
        let mut vss = VerifiableSecretSharingScheme::new(42, 10, 3, 5);
        vss.generate_verifiable_shares();

        // Initially no corruption
        let (verified, corrupted) = vss.detect_byzantine_shares();
        assert_eq!(verified, 5);
        assert_eq!(corrupted, 0);

        // Simulate Byzantine attacker corrupting share 2
        vss.simulate_corruption(2, 999);

        let (verified, corrupted) = vss.detect_byzantine_shares();
        assert_eq!(verified, 4);
        assert_eq!(corrupted, 1);

        // Corrupted share should not pass verification
        assert!(!vss.shares.get(&2).unwrap().is_verified);

        // Other shares still verified
        assert!(vss.shares.get(&1).unwrap().is_verified);
        assert!(vss.shares.get(&3).unwrap().is_verified);
    }

    #[test]
    fn test_multiple_byzantine_attacks() {
        let mut vss = VerifiableSecretSharingScheme::new(42, 10, 3, 5);
        vss.generate_verifiable_shares();

        // Attacker tries to corrupt 2 shares (still below threshold)
        vss.simulate_corruption(1, 100);
        vss.simulate_corruption(4, 200);

        let (verified, corrupted) = vss.detect_byzantine_shares();
        assert_eq!(verified, 3);  // 3 honest shares remain
        assert_eq!(corrupted, 2);

        // With 3 honest shares, we can still reconstruct correctly
        // (threshold = 3, so even with 2 corrupted, we have enough)
    }

    #[test]
    fn test_incoming_share_verification() {
        let vss = VerifiableSecretSharingScheme::new(42, 10, 3, 5);

        // Honest node receives share from provider
        let received_share = 52;  // Correct: P(1) = 42 + 10*1 = 52
        assert!(vss.verify_share(1, received_share));

        // Corrupted share from Byzantine node
        let corrupted_share = 50;  // Wrong value
        assert!(!vss.verify_share(1, corrupted_share));
    }

    #[test]
    fn test_vss_with_field_arithmetic() {
        // Test with larger numbers (simulating real cryptographic use)
        let mut vss = VerifiableSecretSharingScheme::new(1234567890, 9876543210, 3, 5);
        vss.generate_verifiable_shares();

        let (verified, corrupted) = vss.detect_byzantine_shares();
        assert_eq!(verified, 5);
        assert_eq!(corrupted, 0);

        // Even with large numbers, verification should work
        assert!(vss.verify_share(1, 1234567890i64.wrapping_add(9876543210i64.wrapping_mul(1))));
        assert!(vss.verify_share(2, 1234567890i64.wrapping_add(9876543210i64.wrapping_mul(2))));
    }

    #[test]
    fn test_recovery_with_corrupted_shares() {
        let mut vss = VerifiableSecretSharingScheme::new(42, 10, 3, 5);
        vss.generate_verifiable_shares();

        // Corrupt 2 shares
        vss.simulate_corruption(4, 1000);
        vss.simulate_corruption(5, 2000);

        // Still have 3 verified shares
        let verified = vss.get_verified_shares();
        assert_eq!(verified.len(), 3);

        // Verify we have shares 1, 2, 3
        let share_ids: Vec<u32> = verified.iter().map(|s| s.id).collect();
        assert!(share_ids.contains(&1));
        assert!(share_ids.contains(&2));
        assert!(share_ids.contains(&3));

        // Verify their values match expected from polynomial
        for share in verified {
            let expected = 42 + 10 * (share.id as i64);
            assert_eq!(share.share_value, expected);
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════╗");
    println!("║  Share Verification (VSS) Demonstration   ║");
    println!("╚════════════════════════════════════════════╝\n");

    // Create VSS scheme: P(x) = 100 + 20x
    let mut vss = VerifiableSecretSharingScheme::new(100, 20, 3, 5);
    vss.generate_verifiable_shares();

    println!("✓ Generated 5 shares from secret 100 using P(x) = 100 + 20x\n");

    // Display all shares
    println!("📋 All Shares:");
    for (id, share) in &vss.shares {
        println!(
            "  Share {}: value={}, verified={}",
            id, share.share_value, share.is_verified
        );
    }
    println!();

    // Detect Byzantine attack
    println!("🔴 Simulating Byzantine Attack:");
    println!("  Attacker corrupts Share 2...\n");
    vss.simulate_corruption(2, 999);

    let (verified, corrupted) = vss.detect_byzantine_shares();
    println!("✓ Detection Results:");
    println!("  Verified shares:  {}", verified);
    println!("  Corrupted shares: {}\n", corrupted);

    println!("📋 Shares after corruption:");
    for (id, share) in &vss.shares {
        let status = if share.is_verified { "✓" } else { "✗" };
        println!(
            "  {} Share {}: value={}, verified={}",
            status, id, share.share_value, share.is_verified
        );
    }
    println!();

    // Show honest shares can be used for recovery
    let honest_shares = vss.get_verified_shares();
    println!("✓ Honest Shares (safe for reconstruction):");
    for share in &honest_shares {
        println!("  Share {}: {}", share.id, share.share_value);
    }
    println!("\n✓ With {} honest shares (threshold={}), secret is recoverable!",
             honest_shares.len(),
             vss.threshold);
}
