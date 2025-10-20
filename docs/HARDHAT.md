# Hardhat Setup & Testing Guide

## Overview

This guide covers the complete Hardhat development setup for the Blocksense FHE Oracle smart contract.

---

## 1. Installation & Setup

### 1.1 Prerequisites

```bash
# Node.js 16+ required
node --version  # Should be v16+

# npm or yarn
npm --version
```

### 1.2 Install Dependencies

```bash
# Navigate to contracts directory
cd contracts

# Install Hardhat and dependencies
npm install

# Verify installation
npx hardhat --version
```

### 1.3 Environment Configuration

```bash
# Copy example environment file
cp .env.example .env

# Edit .env with your settings
# - PRIVATE_KEY: Your Ethereum private key (for deployments)
# - INFURA_KEY: Get from https://infura.io
# - ETHERSCAN_KEY: Get from https://etherscan.io
```

**Important**: Never commit `.env` with real keys. Use `.env.example` for templates only.

---

## 2. Solidity Compilation

### Compile Contracts

```bash
npm run compile

# Verbose output
npx hardhat compile --verbose

# Clean and recompile
npm run clean
npm run compile
```

**Output**:
- `artifacts/`: Compiled contract ABIs and bytecode
- `typechain-types/`: TypeScript types (if using TypeScript)

---

## 3. Running Tests

### 3.1 Basic Test Execution

```bash
# Run all tests
npm test

# Run tests with verbose output
npm run test:verbose

# Run specific test file
npx hardhat test test/FHEOracle.test.js

# Run specific test suite
npx hardhat test --grep "Event Creation"
```

### 3.2 Test Coverage Report

```bash
# Generate coverage report
npx hardhat coverage

# Output:
# - Coverage report in terminal
# - HTML report in coverage/index.html
```

### 3.3 Gas Usage Report

```bash
# Enable gas reporting during tests
REPORT_GAS=true npm test

# Output:
# - Gas used per function
# - Report saved to gas-report.txt
```

---

## 4. Local Development Network

### 4.1 Start Local Node

```bash
# Terminal 1: Start Hardhat node
npm run node

# Output:
# - Local node running at http://127.0.0.1:8545
# - 20 pre-funded test accounts
# - Display all account private keys
```

### 4.2 Deploy to Local Network

```bash
# Terminal 2: Deploy contract
npm run deploy:local

# Output:
# - Contract address
# - Deployment gas cost
# - Test results
# - Deployment info saved to deployments/localhost-deployment.json
```

### 4.3 Interact with Local Contract

```javascript
// In another terminal or script:
const hre = require("hardhat");

// Get contract instance
const FHEOracle = await hre.ethers.getContractFactory("FHEOracle");
const deployed = await FHEOracle.attach("0x...contract-address");

// Create event
const eventId = hre.ethers.utils.formatBytes32String("test_event");
const tx = await deployed.createEvent(eventId, 1000, Math.floor(Date.now() / 1000) + 86400);
await tx.wait();

console.log("Event created!");
```

---

## 5. Testnet Deployment (Sepolia)

### 5.1 Prepare for Testnet

```bash
# 1. Get test ETH from faucet
# https://sepolia-faucet.pk910.de/

# 2. Set environment variables
# SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_PROJECT_ID
# PRIVATE_KEY=0x...your_private_key

# 3. Fund the deployment account
# Check balance: https://sepolia.etherscan.io
```

### 5.2 Deploy to Sepolia

```bash
# Deploy
npm run deploy:sepolia

# Output:
# - Contract deployed to Sepolia
# - Address and tx hash
# - Deployment info saved to deployments/sepolia-deployment.json
# - Etherscan verification command
```

### 5.3 Verify on Etherscan

```bash
# After deployment
npx hardhat verify --network sepolia 0x...contract-address

# If constructor args exist:
npx hardhat verify --network sepolia 0x...contract-address "arg1" "arg2"
```

**Verified contracts** appear in Etherscan with readable source code.

---

## 6. Contract Testing Details

### 6.1 Test Structure

Test file: `test/FHEOracle.test.js`

```javascript
describe("FHEOracle", function () {
  describe("Event Creation", function () {
    it("should create a new event", async function () {
      // Arrange
      const eventId = ethers.utils.formatBytes32String("eth_price_2025");
      const threshold = 325000000000n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      // Act
      await fheOracle.createEvent(eventId, threshold, deadline);

      // Assert
      const event = await fheOracle.events(eventId);
      expect(event.threshold).to.equal(threshold);
    });
  });
});
```

### 6.2 Test Categories

| Category | Tests | Purpose |
|----------|-------|---------|
| **Event Creation** | 3 | Verify event creation logic |
| **Provider Submissions** | 4 | Test prediction submission |
| **Result Proposal** | 2 | Validate result proposal |
| **Result Finalization** | 2 | Check finalization flow |
| **Dispute Resolution** | 3 | Test dispute & jury voting |
| **Gas Efficiency** | 2 | Monitor gas consumption |
| **Edge Cases** | 3 | Handle boundary conditions |
| **Total** | **19** | Comprehensive coverage |

### 6.3 Running Specific Tests

```bash
# Run single test file
npx hardhat test test/FHEOracle.test.js

# Run tests matching pattern
npx hardhat test --grep "Event Creation"

# Run with specific reporter
npx hardhat test --reporter json > test-results.json

# Run with timeout override
npx hardhat test --timeout 60000
```

---

## 7. Contract Flattening

### Flatten for Etherscan Verification

```bash
npm run flatten

# Output: contracts/FHEOracle.flat.sol
# Use this when uploading to Etherscan
```

---

## 8. Hardhat Console

### Interactive Testing

```bash
# Start Hardhat console
npx hardhat console

# Then in console:
> const contract = await ethers.getContractAt("FHEOracle", "0x...");
> const event = await contract.events(ethers.utils.formatBytes32String("test"));
> console.log(event);

> await contract.createEvent(...)
> process.exit()
```

---

## 9. Network Configuration

### Supported Networks

| Network | Chain ID | Config | Usage |
|---------|----------|--------|-------|
| **Hardhat** | 31337 | Default | Testing (in-memory) |
| **Localhost** | 31337 | `npm run node` | Local development |
| **Sepolia** | 11155111 | Testnet | Public testnet |
| **Mainnet** | 1 | Production | Mainnet (⚠️ real ETH) |

### Add Custom Network

In `hardhat.config.js`:

```javascript
module.exports = {
  networks: {
    mynetwork: {
      url: process.env.MY_RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
      chainId: 12345,
    }
  }
};
```

---

## 10. Common Tasks

### Compile & Test

```bash
npm run compile && npm test
```

### Full Local Workflow

```bash
# Terminal 1
npm run node

# Terminal 2
npm run compile
npm test
npm run deploy:local
```

### Prepare for Mainnet

```bash
# 1. Run all tests
npm test

# 2. Generate coverage
npx hardhat coverage

# 3. Check gas usage
REPORT_GAS=true npm test

# 4. Audit code
# - Use external tools: OpenZeppelin Defender, etc.

# 5. Deploy to testnet first
npm run deploy:sepolia

# 6. Once verified, deploy to mainnet
npm run deploy:mainnet
```

---

## 11. Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Contract not found | Run `npm run compile` first |
| Out of gas | Increase gas limit in config |
| Network timeout | Check RPC URL and internet connection |
| Private key error | Verify `.env` file and key format (0x prefix) |
| Test timeout | Increase timeout in hardhat.config.js |

### Debug Failing Tests

```bash
# Verbose output
npx hardhat test --verbose

# Enable ethers.js debug logging
DEBUG=ethers npm test

# Use console.log in Solidity
// In contract:
emit Debug(msg.value);
// In test:
expect(receipt.events).to.have.lengthOf(1);
```

---

## 12. Gas Optimization Tips

### Current Gas Usage

From `npm run gas-report`:

```
Gas Costs:
  createEvent: ~120,000 gas
  submitPrediction: ~95,000 gas
  proposeResult: ~80,000 gas
  finalizeResult: ~45,000 gas
```

### Optimization Strategies

- Use `immutable` for constants set in constructor
- Pack struct members efficiently
- Use mapping instead of arrays where possible
- Minimize storage reads/writes

---

## 13. Deployment Checklist

Before mainnet deployment:

- [ ] All tests passing (`npm test`)
- [ ] Coverage report reviewed
- [ ] Gas usage acceptable
- [ ] Code audited internally
- [ ] Contract flattened and verified locally
- [ ] Tested on Sepolia first
- [ ] Private key secure (.env not committed)
- [ ] Upgrade path planned
- [ ] Monitoring set up

---

## 14. References

- **Hardhat Documentation**: https://hardhat.org/docs
- **Ethers.js API**: https://docs.ethers.org/
- **Solidity Docs**: https://docs.soliditylang.org/
- **OpenZeppelin Contracts**: https://docs.openzeppelin.com/contracts/
- **Sepolia Faucet**: https://sepolia-faucet.pk910.de/
- **Etherscan**: https://sepolia.etherscan.io/

---

## 15. Script Reference

```bash
npm run compile              # Compile contracts
npm run test                 # Run all tests
npm run test:verbose        # Verbose test output
npm run clean               # Clean build artifacts
npm run node                # Start local node
npm run deploy:local        # Deploy to localhost
npm run deploy:sepolia      # Deploy to Sepolia testnet
npm run deploy:mainnet      # Deploy to mainnet
npm run gas-report          # Generate gas report
npm run flatten             # Flatten contract
npm run verify              # Verify on Etherscan
```

---

**Status**: ✅ Complete  
**Last Updated**: October 20, 2025  
**Version**: 1.0
