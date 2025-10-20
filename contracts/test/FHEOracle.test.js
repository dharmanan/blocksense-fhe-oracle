const { expect } = require("chai");
const { ethers } = require("hardhat");
const { time } = require("@nomicfoundation/hardhat-network-helpers");

describe("FHEOracle", function () {
  let fheOracle;
  let owner, provider1, provider2, provider3;

  beforeEach(async function () {
    [owner, provider1, provider2, provider3] = await ethers.getSigners();
    const FHEOracle = await ethers.getContractFactory("FHEOracle");
    fheOracle = await FHEOracle.deploy();
    await fheOracle.waitForDeployment();
  });

  describe("Event Creation", function () {
    it("should create a new event", async function () {
      const eventId = ethers.id("eth_price_2025");
      const description = "ETH/USD price prediction";
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;

      const tx = await fheOracle.createEvent(eventId, description, deadline);
      const receipt = await tx.wait();
      
      expect(receipt.status).to.equal(1); // Success
      
      const event = await fheOracle.events(eventId);
      expect(event.exists).to.equal(true);
      expect(event.description).to.equal(description);
      expect(event.deadline).to.equal(deadline);
    });

    it("should reject event with past deadline", async function () {
      const eventId = ethers.id("past_event");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime - 1000;

      await expect(
        fheOracle.createEvent(eventId, "Past event", deadline)
      ).to.be.revertedWith("Deadline must be in future");
    });

    it("should reject duplicate event creation", async function () {
      const eventId = ethers.id("duplicate_event");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;

      await fheOracle.createEvent(eventId, "Test event", deadline);

      await expect(
        fheOracle.createEvent(eventId, "Test event 2", deadline)
      ).to.be.revertedWith("Event already exists");
    });

    it("should only allow admin to create events", async function () {
      const eventId = ethers.id("admin_only");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;

      await expect(
        fheOracle.connect(provider1).createEvent(eventId, "Unauthorized", deadline)
      ).to.be.revertedWith("Only admin can call this");
    });
  });

  describe("Result Proposal", function () {
    let eventId;
    let deadline;

    beforeEach(async function () {
      eventId = ethers.id("proposal_event");
      const latestBlockTime = await time.latest();
      deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Test proposal", deadline);
    });

    it("should propose a result", async function () {
      const result = 50;
      const resultHash = ethers.id("result_hash");

      const tx = await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);
      const receipt = await tx.wait();
      expect(receipt.status).to.equal(1);

      const event = await fheOracle.events(eventId);
      expect(event.proposedResult.result).to.equal(result);
      expect(event.proposedResult.proposer).to.equal(provider1.address);
    });

    it("should reject invalid result value", async function () {
      const result = 101; // Invalid: > 100
      const resultHash = ethers.id("result_hash");

      await expect(
        fheOracle.connect(provider1).proposeResult(eventId, result, resultHash)
      ).to.be.revertedWith("Result must be between 0 and 100");
    });

    it("should reject duplicate proposal", async function () {
      const result = 50;
      const resultHash = ethers.id("result_hash");

      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);

      await expect(
        fheOracle.connect(provider2).proposeResult(eventId, result, resultHash)
      ).to.be.revertedWith("Result already proposed");
    });
  });

  describe("Dispute Resolution", function () {
    let eventId;
    let deadline;

    beforeEach(async function () {
      eventId = ethers.id("dispute_event");
      const latestBlockTime = await time.latest();
      deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Dispute test", deadline);
    });

    it("should dispute a proposed result", async function () {
      const result = 50;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);

      const reason = "Suspicious aggregation";
      const tx = await fheOracle.connect(provider2).disputeResult(eventId, reason);
      const receipt = await tx.wait();
      expect(receipt.status).to.equal(1);

      const event = await fheOracle.events(eventId);
      expect(event.proposedResult.status).to.equal(2); // Disputed
    });

    it("should allow jury votes on disputes", async function () {
      const result = 50;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);
      await fheOracle.connect(provider2).disputeResult(eventId, "Bad aggregation");

      const tx = await fheOracle.connect(provider3).voteOnDispute(eventId, true);
      const receipt = await tx.wait();
      expect(receipt.status).to.equal(1);
    });

    it("should prevent double voting", async function () {
      const result = 50;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);
      await fheOracle.connect(provider2).disputeResult(eventId, "Bad aggregation");
      await fheOracle.connect(provider3).voteOnDispute(eventId, true);

      await expect(
        fheOracle.connect(provider3).voteOnDispute(eventId, false)
      ).to.be.revertedWith("Already voted");
    });
  });

  describe("Result Finalization", function () {
    it("should finalize result after dispute window", async function () {
      const eventId = ethers.id("finalize_event");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Finalization test", deadline);

      const result = 50;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);

      await time.increase(86401);

      const tx = await fheOracle.finalizeResult(eventId);
      const receipt = await tx.wait();
      expect(receipt.status).to.equal(1);

      const event = await fheOracle.events(eventId);
      expect(event.proposedResult.status).to.equal(1); // Finalized
    });

    it("should only allow admin to finalize", async function () {
      const eventId = ethers.id("admin_finalize");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Admin finalize test", deadline);

      const result = 50;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, result, resultHash);

      await time.increase(86401);

      await expect(
        fheOracle.connect(provider1).finalizeResult(eventId)
      ).to.be.revertedWith("Only admin can call this");
    });
  });

  describe("View Functions", function () {
    it("should return event details", async function () {
      const eventId = ethers.id("view_event");
      const description = "View function test";
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;

      await fheOracle.createEvent(eventId, description, deadline);

      const event = await fheOracle.events(eventId);
      expect(event.description).to.equal(description);
      expect(event.exists).to.equal(true);
    });

    it("should return event status", async function () {
      const eventId = ethers.id("status_event");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Status test", deadline);

      const status = await fheOracle.getEventStatus(eventId);
      expect(status).to.equal(0); // Proposed
    });

    it("should return proposed result", async function () {
      const eventId = ethers.id("get_result");
      const latestBlockTime = await time.latest();
      const deadline = latestBlockTime + 86400;
      await fheOracle.createEvent(eventId, "Get result test", deadline);

      const resultValue = 75;
      const resultHash = ethers.id("result_hash");
      await fheOracle.connect(provider1).proposeResult(eventId, resultValue, resultHash);

      const result = await fheOracle.getProposedResult(eventId);
      expect(result).to.equal(resultValue);
    });
  });
});
