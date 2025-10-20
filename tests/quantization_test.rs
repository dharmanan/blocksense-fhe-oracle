/// Quantization test suite
/// 
/// Validates all quantization functions against specification:
/// docs/quantization-spec.md

#[cfg(test)]
mod quantization_tests {
    use super::*;

    // ============================================================================
    // PERCENTAGE MARKET TESTS
    // ============================================================================

    #[test]
    fn test_quantize_percent_balanced_market() {
        // Input: 50.5%, Expected: 5050
        let result = quantize_percent(50.5, 10000);
        assert_eq!(result, Ok(5050), "Balanced market (50.5%)");
    }

    #[test]
    fn test_quantize_percent_high_confidence() {
        // Input: 75.25%, Expected: 7525
        let result = quantize_percent(75.25, 10000);
        assert_eq!(result, Ok(7525), "High confidence (75.25%)");
    }

    #[test]
    fn test_quantize_percent_low_confidence() {
        // Input: 25.0%, Expected: 2500
        let result = quantize_percent(25.0, 10000);
        assert_eq!(result, Ok(2500), "Low confidence (25%)");
    }

    #[test]
    fn test_quantize_percent_very_high() {
        // Input: 99.99%, Expected: 9999
        let result = quantize_percent(99.99, 10000);
        assert_eq!(result, Ok(9999), "Very high confidence (99.99%)");
    }

    #[test]
    fn test_quantize_percent_very_low() {
        // Input: 0.01%, Expected: 1
        let result = quantize_percent(0.01, 10000);
        assert_eq!(result, Ok(1), "Very low confidence (0.01%)");
    }

    #[test]
    fn test_quantize_percent_error_negative() {
        // Input: -1, Should error
        let result = quantize_percent(-1.0, 10000);
        assert!(result.is_err(), "Should reject negative percentage");
    }

    #[test]
    fn test_quantize_percent_error_over_100() {
        // Input: 101, Should error
        let result = quantize_percent(101.0, 10000);
        assert!(result.is_err(), "Should reject percentage > 100");
    }

    #[test]
    fn test_quantize_percent_error_zero() {
        // Input: 0%, Should error (unfalsifiable)
        let result = quantize_percent(0.0, 10000);
        assert!(result.is_err(), "Should reject 0% (certain no)");
    }

    #[test]
    fn test_quantize_percent_error_hundred() {
        // Input: 100%, Should error (unfalsifiable)
        let result = quantize_percent(100.0, 10000);
        assert!(result.is_err(), "Should reject 100% (certain yes)");
    }

    // ============================================================================
    // PRICE MARKET TESTS
    // ============================================================================

    #[test]
    fn test_quantize_price_ethereum_like() {
        // Input: $3,250.50, Expected: 325050000000
        let result = quantize_price(3250.50, 8);
        assert_eq!(result, Ok(325050000000), "Ethereum-like price ($3,250.50)");
    }

    #[test]
    fn test_quantize_price_dollar() {
        // Input: $1.00, Expected: 100000000
        let result = quantize_price(1.00, 8);
        assert_eq!(result, Ok(100000000), "Dollar price ($1.00)");
    }

    #[test]
    fn test_quantize_price_penny() {
        // Input: $0.01, Expected: 1000000
        let result = quantize_price(0.01, 8);
        assert_eq!(result, Ok(1000000), "Penny price ($0.01)");
    }

    #[test]
    fn test_quantize_price_minimum_unit() {
        // Input: $0.00000001, Expected: 1
        let result = quantize_price(0.00000001, 8);
        assert_eq!(result, Ok(1), "Minimum unit ($0.00000001)");
    }

    #[test]
    fn test_quantize_price_bitcoin_like() {
        // Input: $50,000, Expected: 5000000000000
        let result = quantize_price(50000.0, 8);
        assert_eq!(result, Ok(5000000000000), "Bitcoin-like price ($50,000)");
    }

    #[test]
    fn test_quantize_price_near_max() {
        // Input: $92,233,720, Expected: near i64::MAX
        let result = quantize_price(92233720.0, 8);
        assert!(result.is_ok(), "Should accept near-max price");
        if let Ok(val) = result {
            assert!(val > 0 && val < i64::MAX, "Value should be within i64 bounds");
        }
    }

    #[test]
    fn test_quantize_price_error_negative() {
        // Input: -1, Should error
        let result = quantize_price(-1.0, 8);
        assert!(result.is_err(), "Should reject negative price");
    }

    #[test]
    fn test_quantize_price_error_exceeds_max() {
        // Input: $92,233,720.37, Should error (exceeds i64 max)
        let result = quantize_price(92233720.37, 8);
        assert!(result.is_err(), "Should reject price exceeding i64 max");
    }

    #[test]
    fn test_quantize_price_error_infinity() {
        // Input: Infinity, Should error
        let result = quantize_price(f64::INFINITY, 8);
        assert!(result.is_err(), "Should reject infinity");
    }

    #[test]
    fn test_quantize_price_error_nan() {
        // Input: NaN, Should error
        let result = quantize_price(f64::NAN, 8);
        assert!(result.is_err(), "Should reject NaN");
    }

    // ============================================================================
    // RATIO MARKET TESTS
    // ============================================================================

    #[test]
    fn test_quantize_ratio_ethereum_prediction() {
        // Input: 0.527, Expected: 527000
        let result = quantize_ratio(0.527, 1000000);
        assert_eq!(result, Ok(527000), "Probability ratio (52.7%)");
    }

    #[test]
    fn test_quantize_ratio_fair_odds() {
        // Input: 0.5, Expected: 500000
        let result = quantize_ratio(0.5, 1000000);
        assert_eq!(result, Ok(500000), "Fair odds (50%)");
    }

    #[test]
    fn test_quantize_ratio_very_likely() {
        // Input: 0.99, Expected: 990000
        let result = quantize_ratio(0.99, 1000000);
        assert_eq!(result, Ok(990000), "Very likely (99%)");
    }

    #[test]
    fn test_quantize_ratio_unlikely() {
        // Input: 0.01, Expected: 10000
        let result = quantize_ratio(0.01, 1000000);
        assert_eq!(result, Ok(10000), "Unlikely (1%)");
    }

    #[test]
    fn test_quantize_ratio_minimum_nonzero() {
        // Input: 0.000001, Expected: 1
        let result = quantize_ratio(0.000001, 1000000);
        assert_eq!(result, Ok(1), "Minimum non-zero ratio");
    }

    #[test]
    fn test_quantize_ratio_certainty() {
        // Input: 1.0, Expected: 1000000
        let result = quantize_ratio(1.0, 1000000);
        assert_eq!(result, Ok(1000000), "Certainty (100%)");
    }

    #[test]
    fn test_quantize_ratio_error_negative() {
        // Input: -0.1, Should error
        let result = quantize_ratio(-0.1, 1000000);
        assert!(result.is_err(), "Should reject negative ratio");
    }

    #[test]
    fn test_quantize_ratio_error_exceeds_one() {
        // Input: 1.1, Should error
        let result = quantize_ratio(1.1, 1000000);
        assert!(result.is_err(), "Should reject ratio > 1.0");
    }

    #[test]
    fn test_quantize_ratio_error_infinity() {
        // Input: Infinity, Should error
        let result = quantize_ratio(f64::INFINITY, 1000000);
        assert!(result.is_err(), "Should reject infinity");
    }

    #[test]
    fn test_quantize_ratio_error_nan() {
        // Input: NaN, Should error
        let result = quantize_ratio(f64::NAN, 1000000);
        assert!(result.is_err(), "Should reject NaN");
    }

    // ============================================================================
    // PRECISION & ROUNDING TESTS
    // ============================================================================

    #[test]
    fn test_quantize_percent_rounding_up() {
        // 50.555% should round to 5056 (not 5055)
        let result = quantize_percent(50.555, 10000);
        assert_eq!(result, Ok(5056), "Proper rounding (50.555% → 5056)");
    }

    #[test]
    fn test_quantize_percent_rounding_down() {
        // 50.544% should round to 5054 (not 5055)
        let result = quantize_percent(50.544, 10000);
        assert_eq!(result, Ok(5054), "Proper rounding (50.544% → 5054)");
    }

    #[test]
    fn test_quantize_price_precision_loss_acceptable() {
        // $3.123456789 → $3.12345679 (loses last digit)
        let input = 3.123456789;
        let result = quantize_price(input, 8);
        assert!(result.is_ok());
        // Verify precision loss is < 1%
        if let Ok(quantized) = result {
            let dequantized = quantized as f64 / 1e8;
            let error_percent = ((input - dequantized).abs() / input) * 100.0;
            assert!(error_percent < 1.0, "Precision loss should be < 1%");
        }
    }

    // ============================================================================
    // INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_fhe_aggregation_example() {
        // Ethereum price prediction by 3 providers
        let provider_a = quantize_price(3250.50, 8).unwrap();  // $3250.50
        let provider_b = quantize_price(3248.50, 8).unwrap();  // $3248.50
        let provider_c = quantize_price(3252.50, 8).unwrap();  // $3252.50

        // Simple averaging (in real FHE, this stays encrypted)
        let sum = provider_a + provider_b + provider_c;
        let average = sum / 3;

        // Dequantize for verification
        let dequantized = average as f64 / 1e8;
        assert!((dequantized - 3250.83).abs() < 0.01, "Average should be ~$3250.83");
    }

    #[test]
    fn test_threshold_comparison_example() {
        // Market: "Will ETH be > $3000?"
        let threshold = quantize_price(3000.0, 8).unwrap();
        let actual_price = quantize_price(3250.50, 8).unwrap();

        assert!(actual_price > threshold, "Price should exceed threshold");
    }
}

// Implementation (must be in main module)
fn quantize_percent(percent: f64, scale: i32) -> Result<i32, String> {
    if percent < 0.0 || percent > 100.0 {
        return Err(format!("Percentage out of range [0, 100]: {}", percent));
    }

    let quantized = (percent * (scale as f64 / 100.0)).round() as i32;

    if quantized == 0 || quantized == scale {
        return Err("Quantized value is extreme (unfalsifiable)".to_string());
    }

    Ok(quantized)
}

fn quantize_price(price: f64, decimals: u32) -> Result<i64, String> {
    if price < 0.0 {
        return Err(format!("Price cannot be negative: {}", price));
    }

    if !price.is_finite() {
        return Err("Price must be finite (not NaN or Infinity)".to_string());
    }

    const MAX_PRICE: f64 = 92233720.36;
    if price > MAX_PRICE {
        return Err(format!("Price exceeds maximum [{}]: {}", MAX_PRICE, price));
    }

    let factor = 10_f64.powi(decimals as i32);
    let quantized = (price * factor).round() as i64;

    Ok(quantized)
}

fn quantize_ratio(ratio: f64, scale: i32) -> Result<i32, String> {
    if ratio < 0.0 || ratio > 1.0 {
        return Err(format!("Ratio out of range [0.0, 1.0]: {}", ratio));
    }

    if !ratio.is_finite() {
        return Err("Ratio must be finite (not NaN or Infinity)".to_string());
    }

    let quantized = (ratio * (scale as f64)).round() as i32;

    Ok(quantized)
}
