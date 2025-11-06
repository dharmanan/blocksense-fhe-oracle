/// Week 4: Threshold Decryption Module
///
/// Implements Shamir's Secret Sharing for distributed decryption
/// Requirements:
/// - 3-of-5 threshold scheme: Any 3 of 5 decryptors can decrypt
/// - Key shares generation
/// - Lagrange interpolation
/// - Threshold decryption

use std::fmt;

/// Shamir's Secret Share
/// Represents one share of a secret split among multiple parties
#[derive(Clone, Debug)]
pub struct SecretShare {
    pub id: u32,                    // Share ID (1 to n)
    pub share_value: i64,           // Actual share value
    pub public_commitment: Vec<u8>, // Public commitment for verification
}

/// Threshold Decryption Configuration
#[derive(Clone, Debug)]
pub struct ThresholdConfig {
    pub total_shares: u32,    // Total number of key shares (n)
    pub threshold: u32,       // Minimum shares needed to decrypt (k)
    pub scheme: String,       // "shamir_3_of_5", "shamir_2_of_3", etc.
}

impl ThresholdConfig {
    pub fn new_3_of_5() -> Self {
        ThresholdConfig {
            total_shares: 5,
            threshold: 3,
            scheme: "shamir_3_of_5".to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.threshold > self.total_shares {
            return Err(format!(
                "Threshold ({}) cannot be greater than total shares ({})",
                self.threshold, self.total_shares
            ));
        }
        if self.threshold < 2 {
            return Err("Threshold must be at least 2".to_string());
        }
        Ok(())
    }
}

/// Decryptor node that holds one key share
#[derive(Clone, Debug)]
pub struct Decryptor {
    pub id: u32,
    pub name: String,
    pub key_share: SecretShare,
}

impl Decryptor {
    pub fn new(id: u32, name: &str, share: SecretShare) -> Self {
        Decryptor {
            id,
            name: name.to_string(),
            key_share: share,
        }
    }
}

/// Threshold Decryption Scheme
pub struct ThresholdScheme {
    pub config: ThresholdConfig,
    pub decryptors: Vec<Decryptor>,
}

impl ThresholdScheme {
    pub fn new(config: ThresholdConfig) -> Result<Self, String> {
        config.validate()?;
        Ok(ThresholdScheme {
            config,
            decryptors: Vec::new(),
        })
    }

    /// Register a decryptor with their key share
    pub fn register_decryptor(&mut self, decryptor: Decryptor) -> Result<(), String> {
        if self.decryptors.len() >= self.config.total_shares as usize {
            return Err(format!(
                "Cannot register more than {} decryptors",
                self.config.total_shares
            ));
        }
        self.decryptors.push(decryptor);
        Ok(())
    }

    /// Check if we have enough decryptors to perform threshold decryption
    pub fn can_decrypt(&self) -> bool {
        (self.decryptors.len() as u32) >= self.config.threshold
    }

    /// Get participating decryptors (first threshold of them)
    pub fn get_participants(&self) -> Vec<&Decryptor> {
        self.decryptors
            .iter()
            .take(self.config.threshold as usize)
            .collect()
    }
}

/// Lagrange Basis Polynomial evaluation
/// Used for polynomial interpolation in Shamir's Secret Sharing
pub fn lagrange_coefficient(
    decryptor_id: u32,
    participants: &[u32],
    modulus: i64,
) -> Result<i64, String> {
    // L_i(0) = product of (-x_j / (x_i - x_j)) for all j != i
    
    let mut numerator = 1i64;
    let mut denominator = 1i64;

    for &participant_id in participants {
        if participant_id != decryptor_id {
            // numerator *= -participant_id
            numerator = numerator.wrapping_mul(-i64::from(participant_id));
            
            // denominator *= (decryptor_id - participant_id)
            let diff = i64::from(decryptor_id) - i64::from(participant_id);
            denominator = denominator.wrapping_mul(diff);
        }
    }

    // Compute modular inverse of denominator
    // For now, using simple computation (in production, use mod_inverse)
    if denominator == 0 {
        return Err("Division by zero in Lagrange coefficient".to_string());
    }

    let coefficient = (numerator as f64 / denominator as f64) as i64;
    Ok(coefficient % modulus)
}

/// Perform threshold decryption using Shamir's Secret Sharing
/// Combines shares from threshold decryptors to recover the result
pub fn threshold_decrypt(
    scheme: &ThresholdScheme,
    encrypted_value: i64,
    modulus: i64,
) -> Result<i64, String> {
    if !scheme.can_decrypt() {
        return Err(format!(
            "Need {} decryptors, only have {}",
            scheme.config.threshold,
            scheme.decryptors.len()
        ));
    }

    let participants: Vec<u32> = scheme
        .get_participants()
        .iter()
        .map(|d| d.id)
        .collect();

    let mut result = 0i64;

    for participant in scheme.get_participants() {
        let coeff = lagrange_coefficient(participant.id, &participants, modulus)?;
        let contribution = participant.key_share.share_value.wrapping_mul(coeff);
        result = result.wrapping_add(contribution);
    }

    // Normalize result to modulus
    result = result % modulus;
    if result < 0 {
        result += modulus;
    }

    Ok(result)
}

/// Distributed key generation (mock version)
/// In production, this uses DKG (Distributed Key Generation) protocol
pub fn generate_key_shares(
    secret: i64,
    config: &ThresholdConfig,
) -> Result<Vec<SecretShare>, String> {
    config.validate()?;

    // Mock polynomial: P(x) = secret + a1*x + a2*x^2 + ... (simplified)
    // For now, generate pseudo-random shares
    let mut shares = Vec::new();

    for i in 1..=config.total_shares {
        let share_id = i;
        // Mock: simple linear scheme P(x) = secret + x*coeff
        let coefficient = 42i64; // In production: use random coefficients
        let share_value = secret + (i as i64) * coefficient;

        shares.push(SecretShare {
            id: share_id,
            share_value,
            public_commitment: vec![], // In production: commitment to polynomial coefficients
        });
    }

    Ok(shares)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_config_validation() {
        let config = ThresholdConfig::new_3_of_5();
        assert!(config.validate().is_ok());

        let bad_config = ThresholdConfig {
            total_shares: 3,
            threshold: 5,
            scheme: "bad".to_string(),
        };
        assert!(bad_config.validate().is_err());
    }

    #[test]
    fn test_decryptor_registration() {
        let config = ThresholdConfig::new_3_of_5();
        let mut scheme = ThresholdScheme::new(config).unwrap();

        let share = SecretShare {
            id: 1,
            share_value: 100,
            public_commitment: vec![],
        };
        let decryptor = Decryptor::new(1, "Decryptor 1", share);

        assert!(scheme.register_decryptor(decryptor).is_ok());
        assert!(!scheme.can_decrypt()); // Need 3 out of 5
    }
}

impl fmt::Display for ThresholdConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Threshold Scheme: {}/{} ({})",
            self.threshold, self.total_shares, self.scheme
        )
    }
}
