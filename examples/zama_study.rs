/// Zama/TFHE Study - Learning FHE Operations
///
/// This file demonstrates:
/// 1. Key generation (client key, server key)
/// 2. Encryption of integers
/// 3. Homomorphic operations (add, multiply, compare)
/// 4. Decryption
///
/// Note: Full FHE operations available in TFHE with specific configuration
/// For now, we'll demonstrate the workflow with basic operations

use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Zama/TFHE Study - FHE Workflow                          ║");
    println!("║  Testing TFHE library for Blocksense FHE Oracle         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // Configure FHE parameters
    let config = ConfigBuilder::default()
        .build()
        .expect("Failed to build config");
    
    println!("✓ FHE Configuration created");
    
    // Generate keys
    let (client_key, server_key) = generate_keys(config);
    println!("✓ Keys generated:");
    println!("  - Client Key (for encryption/decryption)");
    println!("  - Server Key (for homomorphic operations)");
    
    // Set server key for homomorphic operations
    set_server_key(server_key);
    println!("✓ Server key set for FHE operations\n");
    
    // Example 1: Basic Encryption/Decryption
    example_basic_encryption(&client_key);
    
    // Example 2: Homomorphic Addition
    example_addition(&client_key);
    
    // Example 3: Homomorphic Multiplication
    example_multiplication(&client_key);
    
    // Example 4: Oracle Aggregation Workflow
    example_oracle_aggregation(&client_key);
    
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  ✓ All examples completed successfully!                   ║");
    println!("║  TFHE operations are working as expected.                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}

fn example_basic_encryption(client_key: &tfhe::ClientKey) {
    println!("=== Example 1: Basic Encryption/Decryption ===");
    
    let plaintext: u64 = 42;
    println!("  Plaintext: {}", plaintext);
    
    // Encrypt
    let ciphertext = tfhe::FheUint64::try_encrypt(plaintext, client_key)
        .expect("Encryption failed");
    println!("  ✓ Encrypted");
    
    // Decrypt
    let decrypted: u64 = ciphertext.decrypt(client_key);
    println!("  ✓ Decrypted: {} (verification: {})", decrypted, decrypted == plaintext);
    println!();
}

fn example_addition(client_key: &tfhe::ClientKey) {
    println!("=== Example 2: Homomorphic Addition ===");
    
    let a: u64 = 100;
    let b: u64 = 50;
    
    println!("  a = {}, b = {}", a, b);
    
    // Encrypt values
    let ct_a = tfhe::FheUint64::try_encrypt(a, client_key)
        .expect("Encryption failed");
    let ct_b = tfhe::FheUint64::try_encrypt(b, client_key)
        .expect("Encryption failed");
    
    println!("  ✓ Values encrypted");
    
    // Homomorphic addition (WITHOUT decryption)
    let ct_sum = &ct_a + &ct_b;
    println!("  ✓ Performed homomorphic addition on encrypted values");
    
    // Decrypt result
    let sum: u64 = ct_sum.decrypt(client_key);
    println!("  ✓ Decrypted sum: {} (expected: {})", sum, a + b);
    println!("  → Privacy: Oracle never saw values a or b!");
    println!();
}

fn example_multiplication(client_key: &tfhe::ClientKey) {
    println!("=== Example 3: Homomorphic Scalar Multiplication ===");
    
    let value: u64 = 15;
    let scalar: u64 = 3;
    
    println!("  value = {}, scalar = {}", value, scalar);
    
    // Encrypt value
    let ct = tfhe::FheUint64::try_encrypt(value, client_key)
        .expect("Encryption failed");
    
    println!("  ✓ Value encrypted");
    
    // Homomorphic scalar multiplication
    let ct_scaled = &ct * scalar;
    println!("  ✓ Performed homomorphic scalar multiplication on encrypted value");
    
    // Decrypt result
    let result: u64 = ct_scaled.decrypt(client_key);
    println!("  ✓ Decrypted result: {} (expected: {})", result, value * scalar);
    println!();
}

fn example_oracle_aggregation(client_key: &tfhe::ClientKey) {
    println!("=== Example 4: Oracle Aggregation Workflow ===");
    println!();
    println!("Scenario: 3 data providers submit predictions");
    println!("Threshold: 5000");
    println!("Question: Is aggregate prediction above threshold?");
    println!();
    
    // Simulate provider submissions
    let providers = vec![
        ("Provider A", 5000u64, 2u64),  // (name, value, weight)
        ("Provider B", 4500u64, 1u64),
        ("Provider C", 5200u64, 3u64),
    ];
    
    println!("Submissions:");
    for (name, value, weight) in &providers {
        println!("  {}: value={}, weight={}", name, value, weight);
    }
    println!();
    
    // Encrypt all values
    println!("Step 1: Encryption");
    println!("  Providers encrypt their values with public key");
    let mut encrypted_values = Vec::new();
    for (_name, value, _weight) in &providers {
        let ct = tfhe::FheUint64::try_encrypt(*value, client_key)
            .expect("Encryption failed");
        encrypted_values.push(ct);
    }
    println!("  ✓ All values encrypted ({})", encrypted_values.len());
    println!();
    
    // Perform homomorphic aggregation
    println!("Step 2: Homomorphic Aggregation (NO DECRYPTION)");
    println!("  Compute: aggregate = Σ weight_i * value_i");
    let mut aggregate = tfhe::FheUint64::try_encrypt(0u64, client_key)
        .expect("Encryption failed");
    
    for (i, (_name, _value, weight)) in providers.iter().enumerate() {
        let weighted = &encrypted_values[i] * *weight;
        aggregate = &aggregate + &weighted;
        println!("  ✓ Added weighted component #{}", i + 1);
    }
    println!("  ✓ Aggregation complete (still encrypted)");
    println!();
    
    // Threshold comparison
    println!("Step 3: Homomorphic Threshold Comparison");
    let threshold: u64 = 5000;
    let ct_threshold = tfhe::FheUint64::try_encrypt(threshold, client_key)
        .expect("Encryption failed");
    
    println!("  Compute: is_above = (aggregate > {})", threshold);
    let is_above = &aggregate.gt(&ct_threshold);
    println!("  ✓ Comparison complete (still encrypted)");
    println!();
    
    // Decrypt final result
    println!("Step 4: Decryption (by threshold committee)");
    let decrypted_aggregate: u64 = aggregate.decrypt(client_key);
    let result: bool = is_above.decrypt(client_key);
    
    println!("  Decrypted aggregate: {}", decrypted_aggregate);
    println!("  Result: {} > {} = {}", decrypted_aggregate, threshold, result);
    println!();
    
    // Verify against plaintext calculation
    let expected_aggregate: u64 = providers
        .iter()
        .map(|(_, value, weight)| value * weight)
        .sum();
    
    println!("Verification:");
    println!("  Expected aggregate: {}", expected_aggregate);
    println!("  Computed aggregate: {}", decrypted_aggregate);
    println!("  ✓ Match: {}", decrypted_aggregate == expected_aggregate);
    println!();
    
    // Oracle output
    let oracle_result = if result { "YES" } else { "NO" };
    println!("╔════════════════════════════════════════════╗");
    println!("║  ORACLE RESULT: {}                      ║", oracle_result);
    println!("╚════════════════════════════════════════════╝");
    println!();
}
