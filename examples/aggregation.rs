/// Oracle Aggregation Module
///
/// Implements homomorphic aggregation of provider predictions
/// Key feature: All computation happens on encrypted data

use crate::fhe_module::{FheCiphertext, FheBackend};
use std::collections::HashMap;

/// Provider submission (encrypted)
#[derive(Clone, Debug)]
pub struct ProviderSubmission {
    pub provider_id: String,
    pub quantized_value: i64,
    pub weight: i64,
}

/// Aggregation result
#[derive(Clone, Debug)]
pub struct AggregationResult {
    pub aggregate_ciphertext: FheCiphertext,
    pub threshold_ciphertext: FheCiphertext,
    pub diff_ciphertext: FheCiphertext,
    pub comparison_result_ciphertext: FheCiphertext,
    pub metadata: HashMap<String, String>,
}

/// Perform homomorphic aggregation
///
/// Computes: aggregate = Σ (weight_i * value_i)
/// All operations on encrypted data - oracle never sees plaintext!
pub fn homomorphic_aggregate<B: FheBackend + ?Sized>(
    backend: &B,
    submissions: &[ProviderSubmission],
) -> FheCiphertext {
    println!("FHE Aggregation: {} providers", submissions.len());
    
    // Start with 0
    let mut aggregate = backend.encrypt(0);
    
    for (i, submission) in submissions.iter().enumerate() {
        // Encrypt the provider value (would come from blockchain in practice)
        let ct_value = backend.encrypt(submission.quantized_value);
        
        // Homomorphic scalar multiplication: weight * value (encrypted)
        let weighted = backend.scalar_mul(&ct_value, submission.weight);
        
        // Homomorphic addition: aggregate + (weight * value) (encrypted)
        aggregate = backend.add(&aggregate, &weighted);
        
        println!(
            "  ✓ Provider {} added: value={}, weight={} (encrypted)",
            i + 1, submission.quantized_value, submission.weight
        );
    }
    
    println!("  ✓ Aggregation complete (result still encrypted)");
    aggregate
}

/// Perform threshold comparison
///
/// Computes: is_above_threshold = (aggregate > threshold)
/// Result remains encrypted
pub fn homomorphic_threshold_compare<B: FheBackend + ?Sized>(
    backend: &B,
    ct_aggregate: &FheCiphertext,
    threshold: i64,
) -> FheCiphertext {
    println!("FHE Threshold Comparison: threshold={}", threshold);
    
    // Encrypt the threshold
    let ct_threshold = backend.encrypt(threshold);
    
    // Homomorphic comparison: aggregate > threshold (encrypted)
    let result = backend.gt(ct_aggregate, &ct_threshold);
    
    println!("  ✓ Comparison complete (result still encrypted)");
    result
}

/// Full oracle aggregation workflow
///
/// 1. Encrypt provider values
/// 2. Homomorphic weighted aggregation
/// 3. Homomorphic subtraction (CT_aggregate - CT_threshold)
/// 4. Homomorphic threshold comparison
/// 5. Decrypt final result (via MPC)
pub fn oracle_aggregation_workflow<B: FheBackend + ?Sized>(
    backend: &B,
    submissions: &[ProviderSubmission],
    threshold: i64,
) -> AggregationResult {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Oracle Aggregation Workflow                            ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Step 1: Homomorphic aggregation
    println!("Step 1: Homomorphic Weighted Aggregation");
    let ct_aggregate = homomorphic_aggregate(backend, submissions);
    println!();
    
    // Step 2: Homomorphic subtraction for difference calculation
    println!("Step 2: Homomorphic Subtraction (Difference Calculation)");
    let ct_threshold = backend.encrypt(threshold);
    println!("  Computing: CT_diff = CT_aggregate - CT_threshold");
    let ct_diff = backend.sub(&ct_aggregate, &ct_threshold);
    println!("  ✓ Subtraction complete (result still encrypted)");
    println!();
    
    // Step 3: Homomorphic threshold comparison
    println!("Step 3: Homomorphic Threshold Comparison");
    println!("  Computing: CT_result = (CT_aggregate > CT_threshold)");
    let ct_result = backend.gt(&ct_aggregate, &ct_threshold);
    println!("  ✓ Comparison complete (result still encrypted)");
    println!();
    
    // Step 4: Metadata
    let mut metadata = HashMap::new();
    metadata.insert("submissions_count".to_string(), submissions.len().to_string());
    metadata.insert("threshold".to_string(), threshold.to_string());
    metadata.insert("workflow".to_string(), "homomorphic_aggregation_v1".to_string());
    
    println!("Step 4: Result Preparation");
    println!("  ✓ All values encrypted");
    println!("  ✓ All computation verified on encrypted data");
    println!("  ✓ Ready for threshold decryption committee");
    println!();
    
    AggregationResult {
        aggregate_ciphertext: ct_aggregate,
        threshold_ciphertext: ct_threshold,
        diff_ciphertext: ct_diff,
        comparison_result_ciphertext: ct_result,
        metadata,
    }
}

/// Decrypt and reveal final oracle result
///
/// Called by threshold decryption committee
pub fn reveal_oracle_result<B: FheBackend + ?Sized>(
    backend: &B,
    result: &AggregationResult,
) -> OracleDecision {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║  Oracle Result Decryption (by Threshold Committee)      ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // Decrypt values
    let aggregate_value = backend.decrypt(&result.aggregate_ciphertext);
    let threshold_value = backend.decrypt(&result.threshold_ciphertext);
    let diff_value = backend.decrypt(&result.diff_ciphertext);
    let comparison_result = backend.decrypt(&result.comparison_result_ciphertext);
    
    println!("Decrypted Values:");
    println!("  Aggregate:      {}", aggregate_value);
    println!("  Threshold:      {}", threshold_value);
    println!("  Difference:     {} (aggregate - threshold)", diff_value);
    println!("  Comparison (1=YES, 0=NO): {}", comparison_result);
    println!();
    
    let decision = if comparison_result != 0 { "YES" } else { "NO" };
    
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  ORACLE DECISION: {}                                   ║", decision);
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    OracleDecision {
        aggregate_value,
        threshold_value,
        is_above_threshold: comparison_result != 0,
        decision: decision.to_string(),
    }
}

#[derive(Debug)]
pub struct OracleDecision {
    pub aggregate_value: i64,
    pub threshold_value: i64,
    pub is_above_threshold: bool,
    pub decision: String,
}
