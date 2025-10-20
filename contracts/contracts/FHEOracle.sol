// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

/**
 * @title FHEOracle
 * @dev Prediction market oracle with FHE aggregation and dispute resolution
 */
contract FHEOracle {
    // State Variables
    address public admin;
    uint256 public constant DISPUTE_WINDOW = 1 days;
    uint256 public juryThreshold = 3;

    // Enums
    enum ResultStatus {
        Proposed,
        Finalized,
        Disputed,
        Resolved
    }

    // Structs
    struct EventResult {
        bool exists;
        ResultStatus status;
        uint8 result;
        uint256 proposalTime;
        uint256 finalizeTime;
        bytes32 resultHash;
        address proposer;
    }

    struct Dispute {
        bool exists;
        address challenger;
        string reason;
        uint256 challengeTime;
    }

    struct Event {
        bool exists;
        string description;
        uint256 creationTime;
        uint256 deadline;
        EventResult proposedResult;
        Dispute dispute;
    }

    // Mappings
    mapping(bytes32 => Event) public events;
    mapping(bytes32 => mapping(address => bool)) public juryVotes;
    mapping(bytes32 => uint256) public juryVoteCount;

    // Events
    event EventCreated(bytes32 indexed eventId, string description, uint256 deadline, uint256 timestamp);
    event ResultProposed(bytes32 indexed eventId, address indexed proposer, uint8 result, bytes32 resultHash, uint256 proposalTime);
    event ResultFinalized(bytes32 indexed eventId, uint8 result, address indexed proposer, uint256 finalizeTime);
    event ResultDisputed(bytes32 indexed eventId, address indexed challenger, string reason, uint256 challengeTime);
    event DisputeResolved(bytes32 indexed eventId, uint8 finalResult);
    event JuryVoted(bytes32 indexed eventId, address indexed juror, bool inFavor);

    // Modifiers
    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can call this");
        _;
    }

    modifier eventExists(bytes32 eventId) {
        require(events[eventId].exists, "Event does not exist");
        _;
    }

    // Constructor
    constructor() {
        admin = msg.sender;
    }

    // External Functions
    function createEvent(bytes32 eventId, string calldata description, uint256 deadline) external onlyAdmin {
        require(!events[eventId].exists, "Event already exists");
        require(deadline > block.timestamp, "Deadline must be in future");

        events[eventId] = Event({
            exists: true,
            description: description,
            creationTime: block.timestamp,
            deadline: deadline,
            proposedResult: EventResult({
                exists: false,
                status: ResultStatus.Proposed,
                result: 0,
                proposalTime: 0,
                finalizeTime: 0,
                resultHash: bytes32(0),
                proposer: address(0)
            }),
            dispute: Dispute({
                exists: false,
                challenger: address(0),
                reason: "",
                challengeTime: 0
            })
        });

        emit EventCreated(eventId, description, deadline, block.timestamp);
    }

    function proposeResult(bytes32 eventId, uint8 result, bytes32 resultHash) external eventExists(eventId) {
        require(result <= 100, "Result must be between 0 and 100");
        require(!events[eventId].proposedResult.exists, "Result already proposed");
        require(block.timestamp <= events[eventId].deadline, "Event deadline passed");

        events[eventId].proposedResult = EventResult({
            exists: true,
            status: ResultStatus.Proposed,
            result: result,
            proposalTime: block.timestamp,
            finalizeTime: 0,
            resultHash: resultHash,
            proposer: msg.sender
        });

        emit ResultProposed(eventId, msg.sender, result, resultHash, block.timestamp);
    }

    function disputeResult(bytes32 eventId, string calldata reason) external eventExists(eventId) {
        EventResult memory proposedResult = events[eventId].proposedResult;
        require(proposedResult.exists, "No result to dispute");
        require(proposedResult.status == ResultStatus.Proposed, "Can only dispute proposed results");
        require(block.timestamp <= proposedResult.proposalTime + DISPUTE_WINDOW, "Dispute window closed");

        events[eventId].proposedResult.status = ResultStatus.Disputed;
        events[eventId].dispute = Dispute({
            exists: true,
            challenger: msg.sender,
            reason: reason,
            challengeTime: block.timestamp
        });

        emit ResultDisputed(eventId, msg.sender, reason, block.timestamp);
    }

    function finalizeResult(bytes32 eventId) external onlyAdmin eventExists(eventId) {
        EventResult storage result = events[eventId].proposedResult;
        require(result.exists, "No result to finalize");
        require(result.status == ResultStatus.Proposed, "Result not in proposed state");
        require(block.timestamp > result.proposalTime + DISPUTE_WINDOW, "Dispute window still open");

        result.status = ResultStatus.Finalized;
        result.finalizeTime = block.timestamp;

        emit ResultFinalized(eventId, result.result, result.proposer, block.timestamp);
    }

    function voteOnDispute(bytes32 eventId, bool inFavor) external eventExists(eventId) {
        EventResult memory result = events[eventId].proposedResult;
        require(result.status == ResultStatus.Disputed, "Not in dispute phase");
        require(block.timestamp <= result.proposalTime + DISPUTE_WINDOW, "Dispute window closed");
        require(!juryVotes[eventId][msg.sender], "Already voted");

        juryVotes[eventId][msg.sender] = true;
        if (inFavor) {
            juryVoteCount[eventId]++;
        }

        emit JuryVoted(eventId, msg.sender, inFavor);
    }

    function resolveDispute(bytes32 eventId) external onlyAdmin eventExists(eventId) {
        EventResult memory result = events[eventId].proposedResult;
        require(result.status == ResultStatus.Disputed, "Not in dispute phase");
        require(block.timestamp > result.proposalTime + DISPUTE_WINDOW, "Dispute window still open");

        events[eventId].proposedResult.status = ResultStatus.Resolved;
        emit DisputeResolved(eventId, result.result);
    }

    // View Functions
    function getEvent(bytes32 eventId) external view returns (Event memory) {
        return events[eventId];
    }

    function getEventStatus(bytes32 eventId) external view returns (ResultStatus) {
        return events[eventId].proposedResult.status;
    }

    function getProposedResult(bytes32 eventId) external view returns (uint8) {
        return events[eventId].proposedResult.result;
    }
}
