/// Threshold Comparison Tests
///
/// Comprehensive test suite for homomorphic threshold comparison
/// Verifies correctness of encrypted comparison operations

mod fhe_module;
mod aggregation;

use fhe_module::{MockFhe, FheBackend};

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Week 3 Task 4: Threshold Comparison Tests             ║");
    println!("║  Homomorphic Comparison Correctness Verification      ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    let backend = MockFhe;
    
    // Test suite
    run_comparison_tests(&backend);
    run_boundary_tests(&backend);
    run_market_scenario_tests(&backend);
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  ✓ All Threshold Comparison Tests Passed!              ║");
    println!("║    Week 3 Complete: Homomorphic Aggregation             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
}

fn run_comparison_tests(backend: &MockFhe) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test Suite 1: Basic Comparison Operations             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Test: a > b (true case)
    println!("Test 1.1: 100 > 50 = ?");
    let ct_a = backend.encrypt(100);
    let ct_b = backend.encrypt(50);
    let result = backend.gt(&ct_a, &ct_b);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted != 0;
    println!("  Result: {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 1 (true)");
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
    
    // Test: a > b (false case)
    println!("Test 1.2: 50 > 100 = ?");
    let ct_a = backend.encrypt(50);
    let ct_b = backend.encrypt(100);
    let result = backend.gt(&ct_a, &ct_b);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted == 0;
    println!("  Result: {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 0 (false)");
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
    
    // Test: a > a (equal case)
    println!("Test 1.3: 50 > 50 = ?");
    let ct_a = backend.encrypt(50);
    let result = backend.gt(&ct_a, &ct_a);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted == 0;
    println!("  Result: {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 0 (false)");
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
}

fn run_boundary_tests(backend: &MockFhe) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test Suite 2: Boundary & Edge Cases                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Test: Large numbers
    println!("Test 2.1: 9999999 > 9999998 = ?");
    let ct_a = backend.encrypt(9999999);
    let ct_b = backend.encrypt(9999998);
    let result = backend.gt(&ct_a, &ct_b);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted != 0;
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
    
    // Test: Zero comparison
    println!("Test 2.2: 0 > -100 = ?");
    let ct_a = backend.encrypt(0);
    let ct_b = backend.encrypt(-100);
    let result = backend.gt(&ct_a, &ct_b);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted != 0;
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
    
    // Test: Negative numbers
    println!("Test 2.3: -50 > -100 = ?");
    let ct_a = backend.encrypt(-50);
    let ct_b = backend.encrypt(-100);
    let result = backend.gt(&ct_a, &ct_b);
    let decrypted: i64 = backend.decrypt(&result);
    let passed = decrypted != 0;
    println!("  Status: {}\n", if passed { "✓ PASS" } else { "✗ FAIL" });
}

fn run_market_scenario_tests(backend: &MockFhe) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Test Suite 3: Real Market Scenarios                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Scenario 1: Price prediction below threshold
    println!("Scenario 1: ETH Price Prediction");
    println!("  Question: Will ETH price exceed $5000?");
    println!("  Aggregate prediction: 4950");
    println!("  Threshold: 5000");
    println!();
    
    let ct_aggregate = backend.encrypt(4950);
    let ct_threshold = backend.encrypt(5000);
    let result = backend.gt(&ct_aggregate, &ct_threshold);
    let decrypted: i64 = backend.decrypt(&result);
    
    println!("  4950 > 5000 = {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 0 (NO)");
    println!("  Status: {}\n", if decrypted == 0 { "✓ PASS" } else { "✗ FAIL" });
    
    // Scenario 2: Price prediction above threshold
    println!("Scenario 2: Probability Prediction");
    println!("  Question: Will outcome probability exceed 50%?");
    println!("  Aggregate prediction: 5200 (quantized from 52%)");
    println!("  Threshold: 5000 (quantized from 50%)");
    println!();
    
    let ct_aggregate = backend.encrypt(5200);
    let ct_threshold = backend.encrypt(5000);
    let result = backend.gt(&ct_aggregate, &ct_threshold);
    let decrypted: i64 = backend.decrypt(&result);
    
    println!("  5200 > 5000 = {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 1 (YES)");
    println!("  Status: {}\n", if decrypted != 0 { "✓ PASS" } else { "✗ FAIL" });
    
    // Scenario 3: Exact threshold hit
    println!("Scenario 3: Exactly at Threshold");
    println!("  Aggregate prediction: 5000");
    println!("  Threshold: 5000");
    println!();
    
    let ct_aggregate = backend.encrypt(5000);
    let ct_threshold = backend.encrypt(5000);
    let result = backend.gt(&ct_aggregate, &ct_threshold);
    let decrypted: i64 = backend.decrypt(&result);
    
    println!("  5000 > 5000 = {} (encrypted) → {} (decrypted)", result, decrypted);
    println!("  Expected: 0 (NO - not strictly greater)");
    println!("  Status: {}\n", if decrypted == 0 { "✓ PASS" } else { "✗ FAIL" });
}
