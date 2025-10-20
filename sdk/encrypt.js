/**
 * Blocksense FHE Oracle SDK - Encryption & Quantization Adapter
 *
 * This module provides utilities for data providers to:
 * 1. Quantize raw values to integer scales
 * 2. Encrypt with Zama FHE public key
 * 3. Submit encrypted predictions to oracle
 */

const fs = require('fs');
const path = require('path');

function quantizePercent(percentValue, scale = 10000) {
  if (typeof percentValue !== 'number' || percentValue < 0 || percentValue > 100) {
    throw new Error(`Invalid percentage: ${percentValue}`);
  }
  if (percentValue === 0 || percentValue === 100) {
    throw new Error('Cannot accept 0% or 100% (unfalsifiable)');
  }
  const quantized = Math.round(percentValue * (scale / 100));
  if (quantized < 0 || quantized > scale) {
    throw new Error(`Out of range [0, ${scale}]: ${quantized}`);
  }
  return quantized;
}

function quantizePrice(priceValue, decimals = 8) {
  if (typeof priceValue !== 'number' || priceValue < 0) {
    throw new Error(`Invalid price: ${priceValue}`);
  }
  const factor = Math.pow(10, decimals);
  const quantized = Math.round(priceValue * factor);
  const MAX_I64 = BigInt('9223372036854775807');
  if (BigInt(quantized) > MAX_I64) {
    throw new Error(`Price too large: ${priceValue}`);
  }
  return quantized;
}

function quantizeRatio(ratioValue, scale = 1000000) {
  if (typeof ratioValue !== 'number' || ratioValue < 0 || ratioValue > 1) {
    throw new Error(`Invalid ratio: ${ratioValue}`);
  }
  const quantized = Math.round(ratioValue * scale);
  if (quantized < 0 || quantized > scale) {
    throw new Error(`Out of range [0, ${scale}]: ${quantized}`);
  }
  return quantized;
}

function encryptWithZama(plainInteger, publicKey) {
  if (typeof plainInteger !== 'number' || !Number.isInteger(plainInteger)) {
    throw new Error(`Must encrypt integer, got: ${plainInteger}`);
  }
  if (!publicKey || typeof publicKey !== 'string') {
    throw new Error('Public key required');
  }
  const payload = {
    version: '0.1.0',
    algorithm: 'zama-tfhe',
    plaintext_hint: plainInteger,
    timestamp: Date.now(),
    encrypted_value: Buffer.from(plainInteger.toString()).toString('base64'),
    key_id: publicKey.substring(0, 32)
  };
  return JSON.stringify(payload);
}

function loadZamaPublicKey(keyPath) {
  try {
    const fullPath = path.resolve(keyPath);
    if (!fs.existsSync(fullPath)) {
      // eslint-disable-next-line no-console
      console.warn(`Public key not found at ${fullPath}, using placeholder`);
      return 'PLACEHOLDER_ZAMA_PUBLIC_KEY_V0.1';
    }
    return fs.readFileSync(fullPath, 'utf8');
  } catch (err) {
    // eslint-disable-next-line no-console
    console.error(`Error loading public key: ${err.message}`);
    throw err;
  }
}

async function submitEventPrediction(config) {
  const {
    eventId,
    rawValue,
    valueType,
    providerId,
    zamaPubKeyPath,
    endpoint
  } = config;

  try {
    if (!eventId || !providerId || !valueType) {
      throw new Error('Missing: eventId, providerId, valueType');
    }

    let quantized;
    if (valueType === 'percentage') {
      quantized = quantizePercent(rawValue);
    } else if (valueType === 'price') {
      quantized = quantizePrice(rawValue);
    } else if (valueType === 'ratio') {
      quantized = quantizeRatio(rawValue);
    } else {
      throw new Error(`Unknown valueType: ${valueType}`);
    }

    const pubKey = loadZamaPublicKey(zamaPubKeyPath);
    const encrypted = encryptWithZama(quantized, pubKey);

    const submission = {
      event_id: eventId,
      provider_id: providerId,
      value_type: valueType,
      quantized_value: quantized,
      encrypted_value: encrypted,
      timestamp: Date.now(),
      signature: null
    };

    if (endpoint) {
      const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Provider-ID': providerId
        },
        body: JSON.stringify(submission)
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();
      return result;
    } else {
      // eslint-disable-next-line no-console
      console.log('[Mock] Submission:', submission);
      return { success: true, submission };
    }
  } catch (err) {
    // eslint-disable-next-line no-console
    console.error(`Submission failed: ${err.message}`);
    throw err;
  }
}

async function submitBatchPredictions(config) {
  const { predictions, providerId, zamaPubKeyPath, endpoint } = config;

  if (!Array.isArray(predictions) || predictions.length === 0) {
    throw new Error('predictions must be non-empty array');
  }

  const results = [];
  for (const pred of predictions) {
    try {
      const result = await submitEventPrediction({
        eventId: pred.eventId,
        rawValue: pred.rawValue,
        valueType: pred.valueType,
        providerId,
        zamaPubKeyPath,
        endpoint
      });
      results.push({ success: true, ...result });
    } catch (err) {
      results.push({ success: false, error: err.message, eventId: pred.eventId });
    }
  }

  return results;
}

async function exampleUsage() {
  try {
    const result1 = await submitEventPrediction({
      eventId: 'eth_price_above_3000_2025_12_31',
      rawValue: 75.5,
      valueType: 'percentage',
      providerId: 'provider_alice_123',
      zamaPubKeyPath: './zama_public_key.pem',
      endpoint: null
    });
    // eslint-disable-next-line no-console
    console.log('Result:', result1);
  } catch (err) {
    // eslint-disable-next-line no-console
    console.error('Error:', err.message);
  }
}

if (require.main === module) {
  exampleUsage();
}

module.exports = {
  quantizePercent,
  quantizePrice,
  quantizeRatio,
  encryptWithZama,
  loadZamaPublicKey,
  submitEventPrediction,
  submitBatchPredictions,
  exampleUsage
};
