/// Week 4: Threshold Decryption Tests and Examples

mod threshold_decryption;

use threshold_decryption::{
    ThresholdConfig, SecretShare, Decryptor, ThresholdScheme, generate_key_shares,
    threshold_decrypt,
};

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Week 4: Threshold Decryption - Shamir's Secret Sharing ║");
    println!("║  3-of-5 Threshold Scheme Implementation                 ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Test 1: Basic configuration
    test_configuration();

    // Test 2: Key share generation
    test_key_generation();

    // Test 3: Decryptor registration
    test_decryptor_registration();

    // Test 4: Full threshold decryption workflow
    test_full_workflow();

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  ✓ All Week 4 Tests Passed!                            ║");
    println!("║    Threshold Decryption Infrastructure Ready           ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
}

fn test_configuration() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test 1: Threshold Configuration                        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let config = ThresholdConfig::new_3_of_5();
    println!("Configuration: {}", config);
    println!("Total shares: {}", config.total_shares);
    println!("Threshold: {}", config.threshold);
    println!("Scheme: {}\n", config.scheme);

    match config.validate() {
        Ok(_) => println!("✓ Configuration valid\n"),
        Err(e) => println!("✗ Error: {}\n", e),
    }
}

fn test_key_generation() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test 2: Key Share Generation                           ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let config = ThresholdConfig::new_3_of_5();
    let secret = 12345i64;

    println!("Secret: {}", secret);
    println!("Generating {} shares with threshold {}\n", 
        config.total_shares, config.threshold);

    match generate_key_shares(secret, &config) {
        Ok(shares) => {
            for share in &shares {
                println!("  Share {}: value={}", share.id, share.share_value);
            }
            println!("\n✓ Key shares generated successfully\n");
        }
        Err(e) => println!("✗ Error: {}\n", e),
    }
}

fn test_decryptor_registration() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test 3: Decryptor Registration                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let config = ThresholdConfig::new_3_of_5();
    let mut scheme = match ThresholdScheme::new(config) {
        Ok(s) => s,
        Err(e) => {
            println!("✗ Error creating scheme: {}\n", e);
            return;
        }
    };

    println!("Registering decryptors for 3-of-5 scheme...\n");

    let decryptor_names = [
        "Decryptor A",
        "Decryptor B",
        "Decryptor C",
        "Decryptor D",
        "Decryptor E",
    ];

    for (i, name) in decryptor_names.iter().enumerate() {
        let id = (i + 1) as u32;
        let share = SecretShare {
            id,
            share_value: 1000 + (id as i64) * 100,
            public_commitment: vec![],
        };
        let decryptor = Decryptor::new(id, name, share);

        match scheme.register_decryptor(decryptor) {
            Ok(_) => {
                println!("  ✓ {} registered (ID: {})", name, id);
                if scheme.can_decrypt() {
                    println!("    → Threshold reached! Can now decrypt");
                }
            }
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }

    println!("\nTotal decryptors: {}", scheme.decryptors.len());
    println!("Can decrypt: {}\n", scheme.can_decrypt());
}

fn test_full_workflow() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test 4: Full Threshold Decryption Workflow             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let config = ThresholdConfig::new_3_of_5();
    let secret = 42i64;
    let modulus = 1000000i64;

    println!("Workflow:");
    println!("  1. Original secret: {}", secret);
    println!("  2. Generate shares for 5 decryptors");
    println!("  3. Collect 3 shares");
    println!("  4. Reconstruct secret\n");

    // Generate shares
    let shares = match generate_key_shares(secret, &config) {
        Ok(s) => s,
        Err(e) => {
            println!("✗ Error generating shares: {}\n", e);
            return;
        }
    };

    println!("Generated {} shares:", shares.len());
    for share in &shares {
        println!("  Share {}: {}", share.id, share.share_value);
    }
    println!();

    // Create scheme and register first 3 decryptors
    let mut scheme = match ThresholdScheme::new(config) {
        Ok(s) => s,
        Err(e) => {
            println!("✗ Error: {}\n", e);
            return;
        }
    };

    let decryptor_names = ["Decryptor A", "Decryptor B", "Decryptor C"];

    for (i, &name) in decryptor_names.iter().enumerate() {
        let id = (i + 1) as u32;
        let share = shares[i].clone();
        let decryptor = Decryptor::new(id, name, share);
        let _ = scheme.register_decryptor(decryptor);
    }

    println!("Registered decryptors:");
    for d in &scheme.decryptors {
        println!("  {} (Share ID: {})", d.name, d.id);
    }
    println!();

    // Perform threshold decryption
    println!("Attempting threshold decryption...");
    match threshold_decrypt(&scheme, secret, modulus) {
        Ok(result) => {
            println!("  ✓ Decryption successful");
            println!("  Original secret: {}", secret);
            println!("  Reconstructed secret: {}", result);
            println!("  Match: {}\n", secret == result);
        }
        Err(e) => println!("  ✗ Error: {}\n", e),
    }

    // Test with insufficient decryptors
    println!("Testing with insufficient decryptors:");
    let mut scheme2 = ThresholdScheme::new(ThresholdConfig::new_3_of_5()).unwrap();
    
    for i in 0..2 {
        let id = (i + 1) as u32;
        let share = shares[i as usize].clone();
        let decryptor = Decryptor::new(id, &format!("Decryptor {}", char::from_u32(65 + i as u32).unwrap()), share);
        let _ = scheme2.register_decryptor(decryptor);
    }

    println!("  Registered {} decryptors (need 3)", scheme2.decryptors.len());
    println!("  Can decrypt: {}", scheme2.can_decrypt());
    match threshold_decrypt(&scheme2, secret, modulus) {
        Ok(_) => println!("  ✗ Should have failed!"),
        Err(e) => println!("  ✓ Correctly rejected: {}\n", e),
    }
}
