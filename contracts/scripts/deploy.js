const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  console.log("🚀 Deploying FHEOracle contract...\n");

  // Get deployer account
  const [deployer] = await ethers.getSigners();
  console.log(`📝 Deploying contracts with account: ${deployer.address}`);
  console.log(`💰 Account balance: ${ethers.utils.formatEther(await deployer.getBalance())} ETH\n`);

  // Deploy FHEOracle
  const FHEOracle = await ethers.getContractFactory("FHEOracle");
  const fheOracle = await FHEOracle.deploy();
  await fheOracle.deployed();

  console.log(`✅ FHEOracle deployed to: ${fheOracle.address}`);

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`\n📍 Network: ${network.name} (Chain ID: ${network.chainId})`);

  // Save deployment info
  const deploymentInfo = {
    network: network.name,
    chainId: network.chainId,
    deployer: deployer.address,
    contract: fheOracle.address,
    deploymentBlock: await ethers.provider.getBlockNumber(),
    deploymentTime: new Date().toISOString(),
  };

  // Create deployments directory if not exists
  const deploymentsDir = path.join(__dirname, "..", "deployments");
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir, { recursive: true });
  }

  // Save deployment info to file
  const filename = path.join(deploymentsDir, `${network.name}-deployment.json`);
  fs.writeFileSync(filename, JSON.stringify(deploymentInfo, null, 2));
  console.log(`\n📄 Deployment info saved to: ${filename}`);

  // Wait for confirmation if on testnet
  if (network.chainId !== 31337) {
    console.log("\n⏳ Waiting for block confirmations...");
    await fheOracle.deployTransaction.wait(5);
    console.log("✅ Block confirmations received");

    // Prepare verification data
    const verificationInfo = {
      address: fheOracle.address,
      constructorArgs: [],
      network: network.name,
      chainId: network.chainId,
    };

    const verifyFile = path.join(deploymentsDir, `${network.name}-verify.json`);
    fs.writeFileSync(verifyFile, JSON.stringify(verificationInfo, null, 2));
    console.log(`\n🔗 Verify with: npx hardhat verify --network ${network.name} ${fheOracle.address}`);
  }

  // Test basic functionality
  console.log("\n🧪 Testing basic contract functions...");

  try {
    // Create test event
    const eventId = ethers.utils.formatBytes32String("deployment_test");
    const threshold = ethers.BigNumber.from("325000000000"); // $3250
    const deadline = Math.floor(Date.now() / 1000) + 86400; // 1 day

    const tx = await fheOracle.createEvent(eventId, threshold, deadline);
    await tx.wait();
    console.log("  ✓ createEvent() works");

    // Verify event
    const event = await fheOracle.events(eventId);
    console.log(`  ✓ Event stored: threshold=${event.threshold}, state=${event.state}`);

    // Test submission
    const quantizedValue = ethers.BigNumber.from("325050000000");
    const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));

    const submitTx = await fheOracle.submitPrediction(
      eventId,
      quantizedValue,
      encryptedValue
    );
    await submitTx.wait();
    console.log("  ✓ submitPrediction() works");

    console.log("\n✅ All basic tests passed!");
  } catch (error) {
    console.log(`\n⚠️  Basic test failed: ${error.message}`);
    console.log("(This may be expected if the contract has been initialized)");
  }

  console.log("\n📚 Next steps:");
  console.log("  1. Run tests: npm test");
  console.log("  2. Flatten contract: npm run flatten");
  console.log(`  3. Verify on Etherscan: npx hardhat verify --network ${network.name} ${fheOracle.address}`);
  console.log("\n✨ Deployment complete!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Deployment failed:");
    console.error(error);
    process.exit(1);
  });
