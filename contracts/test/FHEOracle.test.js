const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("FHEOracle", function () {
  let fheOracle;
  let owner, provider1, provider2, provider3, decryptor1, decryptor2, decryptor3;

  beforeEach(async function () {
    // Get signers
    [owner, provider1, provider2, provider3, decryptor1, decryptor2, decryptor3] = 
      await ethers.getSigners();

    // Deploy contract
    const FHEOracle = await ethers.getContractFactory("FHEOracle");
    fheOracle = await FHEOracle.deploy();
    await fheOracle.deployed();
  });

  describe("Event Creation", function () {
    it("should create a new event", async function () {
      const eventId = ethers.utils.formatBytes32String("eth_price_2025");
      const threshold = 325000000000n; // $3250
      const deadline = Math.floor(Date.now() / 1000) + 86400; // 1 day

      await expect(
        fheOracle.createEvent(eventId, threshold, deadline)
      ).to.emit(fheOracle, "EventCreated");

      const event = await fheOracle.events(eventId);
      expect(event.threshold).to.equal(threshold);
      expect(event.deadline).to.equal(deadline);
      expect(event.state).to.equal(0); // PENDING
    });

    it("should reject event with past deadline", async function () {
      const eventId = ethers.utils.formatBytes32String("bad_event");
      const threshold = 325000000000n;
      const pastDeadline = Math.floor(Date.now() / 1000) - 3600; // 1 hour ago

      await expect(
        fheOracle.createEvent(eventId, threshold, pastDeadline)
      ).to.revertWith("Invalid deadline");
    });

    it("should reject duplicate event ID", async function () {
      const eventId = ethers.utils.formatBytes32String("eth_price");
      const threshold = 325000000000n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      await expect(
        fheOracle.createEvent(eventId, threshold, deadline)
      ).to.revertWith("Event already exists");
    });
  });

  describe("Provider Submissions", function () {
    let eventId, deadline;

    beforeEach(async function () {
      eventId = ethers.utils.formatBytes32String("eth_price");
      const threshold = 325000000000n;
      deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);
    });

    it("should accept provider prediction", async function () {
      const quantizedValue = 325050000000n; // $3250.50
      const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));

      await expect(
        fheOracle.connect(provider1).submitPrediction(
          eventId,
          quantizedValue,
          encryptedValue
        )
      ).to.emit(fheOracle, "PredictionSubmitted");

      const submission = await fheOracle.getSubmission(eventId, provider1.address);
      expect(submission.quantizedValue).to.equal(quantizedValue);
    });

    it("should reject submission after deadline", async function () {
      const quantizedValue = 325050000000n;
      const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));

      // Advance time past deadline
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");

      await expect(
        fheOracle.connect(provider1).submitPrediction(
          eventId,
          quantizedValue,
          encryptedValue
        )
      ).to.revertWith("Submission period closed");
    });

    it("should allow multiple providers", async function () {
      const providers = [provider1, provider2, provider3];
      const values = [325050000000n, 324850000000n, 325250000000n];

      for (let i = 0; i < providers.length; i++) {
        const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));
        await fheOracle.connect(providers[i]).submitPrediction(
          eventId,
          values[i],
          encryptedValue
        );
      }

      const count = await fheOracle.getSubmissionCount(eventId);
      expect(count).to.equal(3);
    });
  });

  describe("Result Proposal", function () {
    let eventId, threshold, deadline;

    beforeEach(async function () {
      eventId = ethers.utils.formatBytes32String("eth_price");
      threshold = 325000000000n;
      deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      // Submit predictions
      const providers = [provider1, provider2, provider3];
      const values = [325050000000n, 324850000000n, 325250000000n];

      for (let i = 0; i < providers.length; i++) {
        const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));
        await fheOracle.connect(providers[i]).submitPrediction(
          eventId,
          values[i],
          encryptedValue
        );
      }

      // Advance past deadline
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");
    });

    it("should allow proposing result after deadline", async function () {
      const aggregateValue = 325050000000n;
      const result = true; // YES

      await expect(
        fheOracle.proposeResult(eventId, aggregateValue, result)
      ).to.emit(fheOracle, "ResultProposed");

      const event = await fheOracle.events(eventId);
      expect(event.state).to.equal(1); // PROPOSED
    });

    it("should store proposer details", async function () {
      const aggregateValue = 325050000000n;
      const result = true;

      await fheOracle.proposeResult(eventId, aggregateValue, result);

      const event = await fheOracle.events(eventId);
      expect(event.proposedBy).to.equal(owner.address);
      expect(event.proposedResult).to.equal(result);
    });
  });

  describe("Result Finalization", function () {
    let eventId, threshold, deadline;

    beforeEach(async function () {
      eventId = ethers.utils.formatBytes32String("eth_price");
      threshold = 325000000000n;
      deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      // Advance past deadline
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");

      // Propose result
      await fheOracle.proposeResult(eventId, 325050000000n, true);
    });

    it("should finalize result after dispute window", async function () {
      // Advance past dispute window (1 day)
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");

      await expect(
        fheOracle.finalizeResult(eventId)
      ).to.emit(fheOracle, "ResultFinalized");

      const event = await fheOracle.events(eventId);
      expect(event.state).to.equal(2); // FINALIZED
    });

    it("should reject finalization before dispute window", async function () {
      await expect(
        fheOracle.finalizeResult(eventId)
      ).to.revertWith("Dispute window not closed");
    });
  });

  describe("Dispute Resolution", function () {
    let eventId, threshold, deadline;

    beforeEach(async function () {
      eventId = ethers.utils.formatBytes32String("eth_price");
      threshold = 325000000000n;
      deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      // Advance past deadline
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");

      // Propose result
      await fheOracle.proposeResult(eventId, 325050000000n, true);
    });

    it("should allow disputing result", async function () {
      const reason = "Aggregate value incorrect";

      await expect(
        fheOracle.connect(provider1).disputeResult(eventId, reason)
      ).to.emit(fheOracle, "ResultDisputed");

      const event = await fheOracle.events(eventId);
      expect(event.state).to.equal(3); // DISPUTED
    });

    it("should not allow disputing after dispute window", async function () {
      // Advance past dispute window
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine");

      const reason = "Aggregate value incorrect";

      await expect(
        fheOracle.connect(provider1).disputeResult(eventId, reason)
      ).to.revertWith("Dispute window closed");
    });

    it("should allow jury voting", async function () {
      // First dispute
      await fheOracle.connect(provider1).disputeResult(eventId, "Wrong result");

      // Jury votes
      const jurors = [owner, provider1, provider2];
      for (let juror of jurors) {
        await fheOracle.connect(juror).juryVote(eventId, true);
      }

      const event = await fheOracle.events(eventId);
      expect(event.juryVotes).to.equal(3);
    });
  });

  describe("Gas Efficiency", function () {
    it("should create event with reasonable gas", async function () {
      const eventId = ethers.utils.formatBytes32String("gas_test");
      const threshold = 325000000000n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      const tx = await fheOracle.createEvent(eventId, threshold, deadline);
      const receipt = await tx.wait();

      console.log(`  Gas used for createEvent: ${receipt.gasUsed}`);
      expect(receipt.gasUsed).to.be.lessThan(200000); // Should be < 200k
    });

    it("should submit prediction with reasonable gas", async function () {
      const eventId = ethers.utils.formatBytes32String("gas_test");
      const threshold = 325000000000n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      const quantizedValue = 325050000000n;
      const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));

      const tx = await fheOracle.connect(provider1).submitPrediction(
        eventId,
        quantizedValue,
        encryptedValue
      );
      const receipt = await tx.wait();

      console.log(`  Gas used for submitPrediction: ${receipt.gasUsed}`);
      expect(receipt.gasUsed).to.be.lessThan(150000); // Should be < 150k
    });
  });

  describe("Edge Cases", function () {
    it("should handle zero threshold", async function () {
      const eventId = ethers.utils.formatBytes32String("zero_threshold");
      const threshold = 0n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);
      const event = await fheOracle.events(eventId);
      expect(event.threshold).to.equal(0n);
    });

    it("should handle max uint256 threshold", async function () {
      const eventId = ethers.utils.formatBytes32String("max_threshold");
      const threshold = ethers.constants.MaxUint256;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);
      const event = await fheOracle.events(eventId);
      expect(event.threshold).to.equal(threshold);
    });

    it("should handle identical predictions from multiple providers", async function () {
      const eventId = ethers.utils.formatBytes32String("identical");
      const threshold = 325000000000n;
      const deadline = Math.floor(Date.now() / 1000) + 86400;

      await fheOracle.createEvent(eventId, threshold, deadline);

      const value = 325000000000n;
      const providers = [provider1, provider2, provider3];

      for (let provider of providers) {
        const encryptedValue = ethers.utils.hexlify(ethers.utils.randomBytes(32));
        await fheOracle.connect(provider).submitPrediction(
          eventId,
          value,
          encryptedValue
        );
      }

      const count = await fheOracle.getSubmissionCount(eventId);
      expect(count).to.equal(3);
    });
  });
});
