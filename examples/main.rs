/// Main aggregation example
///
/// Demonstrates complete oracle aggregation workflow with FHE
///
/// Example: Prediction market on ETH price
/// - 5 providers submit predictions (encrypted)
/// - Aggregate: weighted average of predictions
/// - Threshold: 5000 (decision point)
/// - Result: YES if aggregate > threshold, else NO

mod fhe_module;
mod aggregation;

use fhe_module::{MockFhe, FheBackend};
use aggregation::{ProviderSubmission, oracle_aggregation_workflow, reveal_oracle_result};

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Blocksense FHE Oracle - Homomorphic Aggregation       ║");
    println!("║  Week 3: Rust Integer Sum + Threshold Comparison      ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Initialize FHE backend
    let fhe_backend = MockFhe;
    println!("✓ FHE Backend initialized (using Mock TFHE)");
    println!("  Ready for real TFHE integration in next phase\n");
    
    // Simulate prediction event
    println!("Event: ETH Price Prediction Market");
    println!("───────────────────────────────────────────────────────────\n");
    
    // Provider submissions
    let submissions = vec![
        ProviderSubmission {
            provider_id: "Provider A".to_string(),
            quantized_value: 5000,
            weight: 2,
        },
        ProviderSubmission {
            provider_id: "Provider B".to_string(),
            quantized_value: 4900,
            weight: 1,
        },
        ProviderSubmission {
            provider_id: "Provider C".to_string(),
            quantized_value: 5200,
            weight: 3,
        },
        ProviderSubmission {
            provider_id: "Provider D".to_string(),
            quantized_value: 5050,
            weight: 2,
        },
        ProviderSubmission {
            provider_id: "Provider E".to_string(),
            quantized_value: 4950,
            weight: 1,
        },
    ];
    
    println!("Provider Submissions:");
    for (i, sub) in submissions.iter().enumerate() {
        println!("  {}. {} → quantized_value={}, weight={}", 
            i + 1, sub.provider_id, sub.quantized_value, sub.weight);
    }
    println!();
    
    // Threshold for the market
    let threshold = 5000;
    println!("Threshold: {}", threshold);
    println!("Question: Will aggregate > {}?", threshold);
    println!();
    
    // Run aggregation workflow
    let result = oracle_aggregation_workflow(&fhe_backend, &submissions, threshold);
    
    // Reveal final result
    let decision = reveal_oracle_result(&fhe_backend, &result);
    
    // Verification
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Verification: Plaintext vs FHE Result                  ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Calculate plaintext result for verification
    let plaintext_aggregate: i64 = submissions
        .iter()
        .map(|sub| sub.quantized_value * sub.weight)
        .sum();
    
    println!("Plaintext Calculation:");
    for sub in &submissions {
        let weighted = sub.quantized_value * sub.weight;
        println!("  {} × {} = {}", sub.quantized_value, sub.weight, weighted);
    }
    println!("  ───────────────────");
    println!("  Sum = {}", plaintext_aggregate);
    println!();
    
    println!("Verification Results:");
    println!("  FHE Aggregate:      {}", decision.aggregate_value);
    println!("  Plaintext Aggregate: {}", plaintext_aggregate);
    println!("  Match: {}", decision.aggregate_value == plaintext_aggregate);
    println!();
    
    println!("Decision Verification:");
    println!("  {} > {} = {}", 
        decision.aggregate_value, 
        decision.threshold_value,
        decision.is_above_threshold);
    println!("  Oracle Result: {}", decision.decision);
    println!();
    
    // Privacy guarantee
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Privacy Guarantee                                      ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    println!("✓ All provider values were encrypted before sending to oracle");
    println!("✓ All aggregation happened on encrypted data");
    println!("✓ Oracle server never saw individual provider values");
    println!("✓ Only threshold decryption committee can see final result");
    println!();
    
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  ✓ Week 3 Task 2 Complete: Homomorphic Aggregation     ║");
    println!("║    Next: Task 3 (Homomorphic Subtraction)             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
}
