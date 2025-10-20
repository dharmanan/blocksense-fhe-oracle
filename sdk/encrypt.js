/**/**// sdk/encrypt.js — Blocksense FHE adapter örneği

 * Blocksense FHE Oracle SDK - Encryption Adapter

 *  * Blocksense FHE Oracle SDK - Encryption Adapter// Data provider → quantize + encrypt (Zama public key)

 * Quantize raw values and encrypt with Zama public key

 */ * // 



const fs = require('fs'); * This module provides utilities for data providers to:// NOTE: Gerçek implementasyon Zama JS SDK entegrasyonu ile yapılacak.

const path = require('path');

 * 1. Quantize raw values to integer scale// Bu dosya yapı ve API göstermek için örnek pseudo-koddur.

/**

 * Quantize percentage to integer scale (0..10000) * 2. Encrypt with Zama public key

 */

function quantizePercent(percentValue, scale = 10000) { * 3. Submit to Blocksense oracle endpointconst fs = require('fs');

  const quantized = Math.round(percentValue * (scale / 100));

  if (quantized < 0 || quantized > scale) { * const path = require('path');

    throw new Error(`Out of range: ${quantized}`);

  } * In production, replace mock Zama functions with actual Zama JS SDK.

  return quantized;

} *//**



/** * Percent (0-100) → Integer (0-10000)

 * Quantize price value

 */const fs = require('fs'); */

function quantizePrice(priceValue, decimals = 2) {

  const factor = Math.pow(10, decimals);const path = require('path');function quantizePercent(p) {

  return Math.round(priceValue * factor);

}  if (p < 0 || p > 100) {



/**/**    throw new Error(`Percent out of range: ${p}`);

 * Mock FHE encryption (replace with actual Zama SDK)

 */ * Quantize a percentage value to integer scale  }

async function encryptWithZamaPublicKey(plainInteger, zamaPubKey) {

  if (typeof plainInteger !== 'number' || !Number.isInteger(plainInteger)) { * Example: 60.5% -> 6050 (scale 0..10000)  return Math.round(p * 100); // 60.5 → 6050

    throw new Error('plainInteger must be integer');

  } */}

  if (!zamaPubKey) {

    throw new Error('zamaPubKey required');function quantizePercent(percentValue, scale = 10000) {

  }

  const quantized = Math.round(percentValue * (scale / 100));/**

  await new Promise(resolve => setTimeout(resolve, 10));

  if (quantized < 0 || quantized > scale) { * Price (USD, any range) → Integer (fixed point)

  const payload = {

    version: 1,    throw new Error(`Quantized value ${quantized} out of range [0, ${scale}]`); * Example: price $1234.56 with 2 decimals → 123456

    algorithm: 'ZAMA_CONCRETE_BOOL',

    plaintext_hint: plainInteger,  } */

    timestamp: Math.floor(Date.now() / 1000),

  };  return quantized;function quantizePrice(price, decimals = 2) {



  const jsonStr = JSON.stringify(payload);}  return Math.round(price * Math.pow(10, decimals));

  const base64Payload = Buffer.from(jsonStr).toString('base64');

  return `ZAMA_CT:v1:CONCRETE_BOOL:${base64Payload}`;}

}

/**

/**

 * Load Zama public key from file * Quantize a continuous value (e.g., price)/**

 */

function loadZamaPublicKey(keyPath) { * Example: 39850.50 USD with scale 100 -> 3985050 (cents) * Probability (0-1) → Integer (0-1000000)

  if (!keyPath) {

    throw new Error('keyPath required'); */ */

  }

function quantizePrice(priceValue, decimals = 2) {function quantizeProbability(prob) {

  try {

    const key = fs.readFileSync(keyPath, 'utf8').trim();  const factor = Math.pow(10, decimals);  if (prob < 0 || prob > 1) {

    if (!key.startsWith('-----BEGIN PUBLIC KEY-----')) {

      console.warn('Warning: Key format unexpected');  return Math.round(priceValue * factor);    throw new Error(`Probability out of range: ${prob}`);

    }

    return key;}  }

  } catch (err) {

    throw new Error(`Failed to load key: ${err.message}`);  return Math.round(prob * 1000000); // 0.605 → 605000

  }

}/**}



/** * Mock FHE encryption (placeholder)

 * Submit event prediction

 */ * In production, use actual Zama/Concrete JS SDK/**

async function submitEventPrediction({

  eventId, *  * Mock: Encrypt with Zama public key

  rawValue,

  valueType = 'percent', * Real signature might be: * 

  providerId,

  zamaPubKeyPath, * const encryptedValue = zamaSdk.encrypt(plainInteger, publicKey); * TODO: Replace with actual Zama JS SDK when available

  endpoint,

}) { */ * Expected API (from Zama docs):

  if (!eventId || !providerId || !zamaPubKeyPath || !endpoint) {

    throw new Error('Missing required parameters');async function encryptWithZamaPublicKey(plainInteger, zamaPubKey) { *   const zama = require('@zama-ai/concrete-js');

  }

  // Validate inputs *   const pubkey = zama.deserializePublicKey(zamaPubKeyPEM);

  let quantizedValue;

  switch (valueType) {  if (typeof plainInteger !== 'number' || !Number.isInteger(plainInteger)) { *   const ct = pubkey.encrypt(plainInt);

    case 'percent':

      quantizedValue = quantizePercent(rawValue);    throw new Error('plainInteger must be an integer'); *   return ct.serialize();

      break;

    case 'price':  } */

      quantizedValue = quantizePrice(rawValue);

      break;  if (!zamaPubKey) {async function encryptWithZamaPublicKey(plainInt, zamaPubKeyPEM) {

    case 'ratio':

      quantizedValue = Math.round(rawValue * 10000);    throw new Error('zamaPubKey is required');  // Mock: Base64-encoded ciphertext for now

      break;

    default:  }  // Actual: Use Zama SDK

      throw new Error(`Unknown valueType: ${valueType}`);

  }  



  console.log(`[${eventId}] Quantized ${valueType} ${rawValue} -> ${quantizedValue}`);  // Mock: simulate encryption delay  console.log('[MOCK] Encrypting plaintext:', plainInt);



  const zamaPubKey = loadZamaPublicKey(zamaPubKeyPath);  await new Promise(resolve => setTimeout(resolve, 10));  

  const ciphertext = await encryptWithZamaPublicKey(quantizedValue, zamaPubKey);

  console.log(`[${eventId}] Encrypted with Zama key`);  // Simulated ciphertext: just base64 encode for demo



  const payload = {  // Mock: return base64-encoded ciphertext  const mockCiphertext = Buffer.from(`CT:${plainInt}:${Date.now()}`).toString('base64');

    eventId,

    providerId,  // Format: "ZAMA_CT:v1:<algorithm>:<base64_payload>"  

    valueCiphertext: ciphertext,

    metadata: {  const payload = {  return mockCiphertext;

      valueType,

      quantization: valueType === 'percent' ? 'scale_10000' : `decimals_${valueType === 'price' ? 2 : 4}`,    version: 1,}

      submissionTime: Math.floor(Date.now() / 1000),

      sdkVersion: '1.0.0',    algorithm: 'ZAMA_CONCRETE_BOOL',

    },

  };    plaintext_hint: plainInteger,/**



  console.log(`[${eventId}] Payload ready for ${endpoint}`);    timestamp: Math.floor(Date.now() / 1000), * Push quantized & encrypted event data to Blocksense FHE oracle

  return payload;

}  }; * 



/** * @param {string} eventId - Unique event identifier

 * Batch submit predictions

 */  const jsonStr = JSON.stringify(payload); * @param {string} providerId - Data provider ID

async function submitBatchPredictions(predictions, zamaPubKeyPath, endpoint) {

  const results = [];  const base64Payload = Buffer.from(jsonStr).toString('base64'); * @param {number} rawValue - Raw data value (e.g., 60.5 for percent)

  for (const pred of predictions) {

    try {  return `ZAMA_CT:v1:CONCRETE_BOOL:${base64Payload}`; * @param {string} quantizationType - 'percent', 'price', 'probability'

      const payload = await submitEventPrediction({

        ...pred,} * @param {string} zamaPubKeyPEM - Zama public key (PEM)

        zamaPubKeyPath,

        endpoint, * @param {string} endpoint - Oracle API endpoint

      });

      results.push({ status: 'success', payload });/** * @returns {object} Payload object

    } catch (err) {

      results.push({ status: 'error', error: err.message }); * Load Zama public key from file or environment */

    }

  } */async function pushEventData(

  return results;

}function loadZamaPublicKey(keyPath) {  eventId,



module.exports = {  if (!keyPath) {  providerId,

  quantizePercent,

  quantizePrice,    throw new Error('keyPath is required');  rawValue,

  encryptWithZamaPublicKey,

  loadZamaPublicKey,  }  quantizationType,

  submitEventPrediction,

  submitBatchPredictions,  zamaPubKeyPEM,

};

  try {  endpoint = 'http://localhost:8080/oracle'

if (require.main === module) {

  (async () => {    const key = fs.readFileSync(keyPath, 'utf8').trim();) {

    console.log('Blocksense FHE Oracle SDK\n');

    if (!key.startsWith('-----BEGIN PUBLIC KEY-----')) {  

    const mockKeyPath = path.join(__dirname, 'zama_pub.key');

    if (!fs.existsSync(mockKeyPath)) {      console.warn('Warning: Public key does not have expected PEM format');  // Quantize based on type

      const mockKey = `-----BEGIN PUBLIC KEY-----

MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...    }  let quantInt;

-----END PUBLIC KEY-----`;

      fs.writeFileSync(mockKeyPath, mockKey);    return key;  switch (quantizationType) {

    }

  } catch (err) {    case 'percent':

    try {

      const payload = await submitEventPrediction({    throw new Error(`Failed to load Zama public key from ${keyPath}: ${err.message}`);      quantInt = quantizePercent(rawValue);

        eventId: 'event_btc_40k_2025q1',

        rawValue: 60.5,  }      break;

        valueType: 'percent',

        providerId: 'provider_alice',}    case 'price':

        zamaPubKeyPath: mockKeyPath,

        endpoint: 'https://oracle.blocksense.local/submit',      quantInt = quantizePrice(rawValue);

      });

/**      break;

      console.log('\nPayload:', JSON.stringify(payload, null, 2));

    } catch (err) { * Main: Submit event prediction from provider    case 'probability':

      console.error('Error:', err.message);

    } *       quantInt = quantizeProbability(rawValue);

  })();

} * @param {object} eventData - Event details      break;


 * @param {string} eventData.eventId - Unique event identifier    default:

 * @param {number} eventData.rawValue - Raw prediction (e.g., 60.5 for 60.5%)      throw new Error(`Unknown quantization type: ${quantizationType}`);

 * @param {string} eventData.valueType - 'percent' | 'price' | 'ratio'  }

 * @param {string} providerId - Provider identifier

 * @param {string} zamaPubKeyPath - Path to Zama public key file  console.log(`Quantized ${quantizationType} value:`, quantInt);

 * @param {string} endpoint - Blocksense endpoint to submit to

 * @returns {object} Encrypted payload ready for submission  // Encrypt

 */  const ciphertext = await encryptWithZamaPublicKey(quantInt, zamaPubKeyPEM);

async function submitEventPrediction({

  eventId,  // Build payload

  rawValue,  const payload = {

  valueType = 'percent',    eventId,

  providerId,    providerId,

  zamaPubKeyPath,    valueCiphertext: ciphertext,

  endpoint,    meta: {

}) {      quantizationType,

  if (!eventId || !providerId || !zamaPubKeyPath || !endpoint) {      quantizedValue: quantInt,

    throw new Error('eventId, providerId, zamaPubKeyPath, and endpoint are required');      timestamp: Math.floor(Date.now() / 1000),

  }      rawValue,

    },

  // Quantize based on type  };

  let quantizedValue;

  switch (valueType) {  console.log('Payload ready:', JSON.stringify(payload, null, 2));

    case 'percent':

      quantizedValue = quantizePercent(rawValue);  // TODO: POST to endpoint

      break;  // const response = await fetch(endpoint, {

    case 'price':  //   method: 'POST',

      quantizedValue = quantizePrice(rawValue);  //   headers: { 'Content-Type': 'application/json' },

      break;  //   body: JSON.stringify(payload),

    case 'ratio':  // });

      quantizedValue = Math.round(rawValue * 10000);

      break;  return payload;

    default:}

      throw new Error(`Unknown valueType: ${valueType}`);

  }/**

 * Main: Demo with sample data

  console.log(`[${eventId}] Quantized ${valueType} ${rawValue} -> ${quantizedValue}`); */

async function main() {

  // Load public key  console.log('=== Blocksense FHE Oracle Adapter ===\n');

  const zamaPubKey = loadZamaPublicKey(zamaPubKeyPath);

  // Load mock Zama public key

  // Encrypt  const zamaPubKeyPath = path.join(__dirname, '../keys/zama_pub.pem');

  const ciphertext = await encryptWithZamaPublicKey(quantizedValue, zamaPubKey);  let zamaPubKey = 'MOCK_ZAMA_PUB_KEY';

  console.log(`[${eventId}] Encrypted with Zama public key`);  

  if (fs.existsSync(zamaPubKeyPath)) {

  // Build payload    zamaPubKey = fs.readFileSync(zamaPubKeyPath, 'utf8');

  const payload = {    console.log('Loaded Zama pub key from:', zamaPubKeyPath);

    eventId,  } else {

    providerId,    console.log('[WARN] Zama pub key not found at', zamaPubKeyPath, '— using mock');

    valueCiphertext: ciphertext,  }

    metadata: {  console.log();

      valueType,

      quantization: valueType === 'percent' ? 'scale_10000' : `decimals_${valueType === 'price' ? 2 : 4}`,  // Sample event data

      submissionTime: Math.floor(Date.now() / 1000),  const sampleEvents = [

      sdkVersion: '1.0.0',    {

    },      eventId: 'event_0x001',

  };      providerId: 'provider_a',

      value: 60.5,

  console.log(`[${eventId}] Payload ready for ${endpoint}`);      type: 'percent',

  // In production: await fetch(endpoint, { method: 'POST', body: JSON.stringify(payload) })    },

  return payload;    {

}      eventId: 'event_0x002',

      providerId: 'provider_b',

/**      value: 1234.56,

 * Batch submit multiple predictions      type: 'price',

 */    },

async function submitBatchPredictions(predictions, zamaPubKeyPath, endpoint) {    {

  const results = [];      eventId: 'event_0x003',

  for (const pred of predictions) {      providerId: 'provider_c',

    try {      value: 0.85,

      const payload = await submitEventPrediction({      type: 'probability',

        ...pred,    },

        zamaPubKeyPath,  ];

        endpoint,

      });  // Process each

      results.push({ status: 'success', payload });  for (const evt of sampleEvents) {

    } catch (err) {    console.log(`\n--- Event: ${evt.eventId} ---`);

      results.push({ status: 'error', error: err.message });    try {

    }      await pushEventData(

  }        evt.eventId,

  return results;        evt.providerId,

}        evt.value,

        evt.type,

// Export functions        zamaPubKey,

module.exports = {      );

  quantizePercent,    } catch (err) {

  quantizePrice,      console.error('Error:', err.message);

  encryptWithZamaPublicKey,    }

  loadZamaPublicKey,  }

  submitEventPrediction,}

  submitBatchPredictions,

};// Export for SDK use

module.exports = {

// Example usage (if run directly)  quantizePercent,

if (require.main === module) {  quantizePrice,

  (async () => {  quantizeProbability,

    console.log('Blocksense FHE Oracle SDK - Example\n');  encryptWithZamaPublicKey,

  pushEventData,

    // Create a mock Zama public key for testing};

    const mockKeyPath = path.join(__dirname, 'zama_pub.key');

    if (!fs.existsSync(mockKeyPath)) {// Run if called directly

      const mockKey = `-----BEGIN PUBLIC KEY-----if (require.main === module) {

MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...  main().catch(console.error);

-----END PUBLIC KEY-----`;}

      fs.writeFileSync(mockKeyPath, mockKey);
    }

    try {
      const payload = await submitEventPrediction({
        eventId: 'event_btc_40k_2025q1',
        rawValue: 60.5,
        valueType: 'percent',
        providerId: 'provider_alice',
        zamaPubKeyPath: mockKeyPath,
        endpoint: 'https://oracle.blocksense.local/submit',
      });

      console.log('\nPayload:', JSON.stringify(payload, null, 2));
    } catch (err) {
      console.error('Error:', err.message);
    }
  })();
}
