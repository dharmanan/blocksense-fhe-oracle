/**
 * Quantization Test Suite
 * 
 * Validates all quantization functions against specification:
 * docs/quantization-spec.md
 */

const assert = require('assert');

// ============================================================================
// IMPLEMENTATION (from sdk/encrypt.js)
// ============================================================================

function quantizePercent(percentValue, scale = 10000) {
  // Validate input
  if (typeof percentValue !== 'number' || !Number.isFinite(percentValue)) {
    throw new Error(`Invalid percentage: ${percentValue}`);
  }

  if (percentValue < 0 || percentValue > 100) {
    throw new Error(`Percentage out of range [0, 100]: ${percentValue}`);
  }

  // Quantize
  const quantized = Math.round(percentValue * (scale / 100));

  // Reject extremes (unfalsifiable)
  if (quantized === 0 || quantized === scale) {
    throw new Error(`Quantized value is extreme (unfalsifiable): ${quantized}`);
  }

  return quantized;
}

function quantizePrice(priceValue, decimals = 8) {
  // Validate input
  if (typeof priceValue !== 'number' || !Number.isFinite(priceValue)) {
    throw new Error(`Invalid price: ${priceValue}`);
  }

  if (priceValue < 0) {
    throw new Error(`Price cannot be negative: ${priceValue}`);
  }

  // Max safe value for i64 with 8 decimals: 9,223,372,036.85
  const MAX_PRICE = 92233720.36;
  if (priceValue > MAX_PRICE) {
    throw new Error(`Price exceeds maximum [${MAX_PRICE}]: ${priceValue}`);
  }

  // Quantize
  const factor = Math.pow(10, decimals);
  const quantized = Math.round(priceValue * factor);

  return quantized;
}

function quantizeRatio(ratioValue, scale = 1000000) {
  // Validate input
  if (typeof ratioValue !== 'number' || !Number.isFinite(ratioValue)) {
    throw new Error(`Invalid ratio: ${ratioValue}`);
  }

  if (ratioValue < 0.0 || ratioValue > 1.0) {
    throw new Error(`Ratio out of range [0.0, 1.0]: ${ratioValue}`);
  }

  // Quantize
  const quantized = Math.round(ratioValue * scale);

  return quantized;
}

// ============================================================================
// PERCENTAGE MARKET TESTS
// ============================================================================

describe('Quantize Percentage', () => {
  it('should quantize balanced market (50.5%)', () => {
    assert.strictEqual(quantizePercent(50.5), 5050);
  });

  it('should quantize high confidence (75.25%)', () => {
    assert.strictEqual(quantizePercent(75.25), 7525);
  });

  it('should quantize low confidence (25%)', () => {
    assert.strictEqual(quantizePercent(25.0), 2500);
  });

  it('should quantize very high confidence (99.99%)', () => {
    assert.strictEqual(quantizePercent(99.99), 9999);
  });

  it('should quantize very low confidence (0.01%)', () => {
    assert.strictEqual(quantizePercent(0.01), 1);
  });

  it('should reject negative percentage', () => {
    assert.throws(() => quantizePercent(-1), /out of range/);
  });

  it('should reject percentage > 100', () => {
    assert.throws(() => quantizePercent(101), /out of range/);
  });

  it('should reject 0% (unfalsifiable)', () => {
    assert.throws(() => quantizePercent(0), /extreme/);
  });

  it('should reject 100% (unfalsifiable)', () => {
    assert.throws(() => quantizePercent(100), /extreme/);
  });

  it('should reject NaN', () => {
    assert.throws(() => quantizePercent(NaN), /Invalid/);
  });

  it('should reject Infinity', () => {
    assert.throws(() => quantizePercent(Infinity), /Invalid/);
  });

  it('should handle proper rounding (50.555% → 5056)', () => {
    assert.strictEqual(quantizePercent(50.555), 5056);
  });

  it('should handle proper rounding (50.544% → 5054)', () => {
    assert.strictEqual(quantizePercent(50.544), 5054);
  });
});

// ============================================================================
// PRICE MARKET TESTS
// ============================================================================

describe('Quantize Price', () => {
  it('should quantize Ethereum-like price ($3,250.50)', () => {
    assert.strictEqual(quantizePrice(3250.50, 8), 325050000000);
  });

  it('should quantize dollar price ($1.00)', () => {
    assert.strictEqual(quantizePrice(1.00, 8), 100000000);
  });

  it('should quantize penny price ($0.01)', () => {
    assert.strictEqual(quantizePrice(0.01, 8), 1000000);
  });

  it('should quantize minimum unit ($0.00000001)', () => {
    assert.strictEqual(quantizePrice(0.00000001, 8), 1);
  });

  it('should quantize Bitcoin-like price ($50,000)', () => {
    assert.strictEqual(quantizePrice(50000.0, 8), 5000000000000);
  });

  it('should accept near-max price ($92,233,720)', () => {
    const result = quantizePrice(92233720.0, 8);
    assert(result > 0 && result < Number.MAX_SAFE_INTEGER);
  });

  it('should reject negative price', () => {
    assert.throws(() => quantizePrice(-1), /negative/);
  });

  it('should reject price exceeding i64 max', () => {
    assert.throws(() => quantizePrice(92233720.37, 8), /exceeds/);
  });

  it('should reject NaN', () => {
    assert.throws(() => quantizePrice(NaN, 8), /Invalid/);
  });

  it('should reject Infinity', () => {
    assert.throws(() => quantizePrice(Infinity, 8), /Invalid/);
  });

  it('should have acceptable precision loss (< 1%)', () => {
    const input = 3.123456789;
    const quantized = quantizePrice(input, 8);
    const dequantized = quantized / 1e8;
    const errorPercent = Math.abs((input - dequantized) / input) * 100;
    assert(errorPercent < 1.0, `Precision loss ${errorPercent}% should be < 1%`);
  });
});

// ============================================================================
// RATIO MARKET TESTS
// ============================================================================

describe('Quantize Ratio', () => {
  it('should quantize probability ratio (52.7%)', () => {
    assert.strictEqual(quantizeRatio(0.527), 527000);
  });

  it('should quantize fair odds (50%)', () => {
    assert.strictEqual(quantizeRatio(0.5), 500000);
  });

  it('should quantize very likely (99%)', () => {
    assert.strictEqual(quantizeRatio(0.99), 990000);
  });

  it('should quantize unlikely (1%)', () => {
    assert.strictEqual(quantizeRatio(0.01), 10000);
  });

  it('should quantize minimum non-zero ratio', () => {
    assert.strictEqual(quantizeRatio(0.000001), 1);
  });

  it('should quantize certainty (100%)', () => {
    assert.strictEqual(quantizeRatio(1.0), 1000000);
  });

  it('should reject negative ratio', () => {
    assert.throws(() => quantizeRatio(-0.1), /out of range/);
  });

  it('should reject ratio > 1.0', () => {
    assert.throws(() => quantizeRatio(1.1), /out of range/);
  });

  it('should reject NaN', () => {
    assert.throws(() => quantizeRatio(NaN), /Invalid/);
  });

  it('should reject Infinity', () => {
    assert.throws(() => quantizeRatio(Infinity), /Invalid/);
  });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Integration Tests', () => {
  it('should aggregate Ethereum price predictions from 3 providers', () => {
    const providerA = quantizePrice(3250.50, 8);
    const providerB = quantizePrice(3248.50, 8);
    const providerC = quantizePrice(3252.50, 8);

    const sum = providerA + providerB + providerC;
    const average = Math.round(sum / 3);
    
    // Dequantize for verification
    const dequantized = average / 1e8;
    
    // Should be close to $3250.83
    assert(Math.abs(dequantized - 3250.83) < 0.01, 
      `Average ${dequantized} should be ~$3250.83`);
  });

  it('should correctly compare with threshold', () => {
    const threshold = quantizePrice(3000.0, 8);
    const actualPrice = quantizePrice(3250.50, 8);
    
    assert(actualPrice > threshold, 'Price should exceed threshold');
  });

  it('should handle market lifecycle', () => {
    // Market creation: Set parameters
    const market = {
      type: 'percentage',
      threshold: quantizePercent(60),
      created_at: Date.now()
    };

    // Provider submissions
    const submission1 = quantizePercent(65.5);
    const submission2 = quantizePercent(58.2);
    const submission3 = quantizePercent(62.1);

    // Aggregate (simple average in plaintext)
    const average = Math.round((submission1 + submission2 + submission3) / 3);

    // Check result
    const result = average >= market.threshold ? 'YES' : 'NO';
    assert.strictEqual(result, 'YES', 'Market should settle YES');
  });

  it('should support custom scales', () => {
    // Use 1000 scale instead of 10000 for percentage
    const custom = quantizePercent(50.5, 1000);
    assert.strictEqual(custom, 505, 'Should scale correctly with custom scale');
  });
});

// ============================================================================
// RUN TESTS (if executed directly)
// ============================================================================

if (require.main === module) {
  console.log('Running quantization tests...\n');
  
  // Simple test runner for debugging
  const tests = [
    ['quantizePercent(50.5)', () => quantizePercent(50.5) === 5050],
    ['quantizePrice(3250.50)', () => quantizePrice(3250.50, 8) === 325050000000],
    ['quantizeRatio(0.527)', () => quantizeRatio(0.527) === 527000],
    ['Reject 0%', () => { try { quantizePercent(0); return false; } catch { return true; } }],
    ['Reject 100%', () => { try { quantizePercent(100); return false; } catch { return true; } }],
  ];

  let passed = 0;
  tests.forEach(([name, test]) => {
    try {
      if (test()) {
        console.log(`✓ ${name}`);
        passed++;
      } else {
        console.log(`✗ ${name}`);
      }
    } catch (e) {
      console.log(`✗ ${name}: ${e.message}`);
    }
  });

  console.log(`\n${passed}/${tests.length} tests passed`);
}

module.exports = {
  quantizePercent,
  quantizePrice,
  quantizeRatio
};
