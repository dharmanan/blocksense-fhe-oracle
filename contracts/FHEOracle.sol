// SPDX-License-Identifier: MIT// SPDX-License-Identifier: MIT// SPDX-License-Identifier: MIT

pragma solidity ^0.8.18;

pragma solidity ^0.8.18;pragma solidity ^0.8.18;

/**

 * @title FHEOracle

 * @dev Prediction market oracle with FHE aggregation and dispute resolution

 *//**/**



contract FHEOracle { * @title FHEOracle * @title FHEOracle

    address public admin;

    uint256 public disputeWindow = 1 days; * @dev Prediction market oracle with FHE-based aggregation and dispute resolution * @dev Blocksense FHE-based prediction market oracle

    

    enum ResultStatus { Proposed, Finalized, Disputed, Resolved } *  * 

    

    struct Event { * Workflow: * Flow:

        bool exists;

        ResultStatus status; * 1. Proposer submits resultHash (hash of aggregated encrypted result) * 1. Data providers encrypt values with Zama public key (off-chain)

        uint8 result;

        uint256 proposalTime; * 2. System waits for MPC decryptors to sign the result * 2. FHE compute aggregates ciphertexts (off-chain)

        uint256 finalizeTime;

        bytes32 resultHash; * 3. Proposer submits result + signatures to finalize * 3. Threshold decrypt produces plaintext result (off-chain, MPC)

        address proposer;

    } * 4. Challengers can dispute within window * 4. Oracle operator proposes result with proof

    

    struct Dispute { * 5. Jury/arbitration resolves disputes * 5. Dispute window (1 day) opens

        bool exists;

        address challenger; * 6. Winners claim payouts * 6. Jury can challenge if suspicious

        string reason;

        uint256 challengeTime; */ * 7. After window, admin/operator finalizes

    }

     */

    mapping(bytes32 => Event) public events;

    mapping(bytes32 => Dispute) public disputes;contract FHEOracle {

    

    event EventCreated(bytes32 indexed eventId, string description, uint256 timestamp);    // ========== State Variables ==========contract FHEOracle {

    event ResultProposed(bytes32 indexed eventId, address indexed proposer, bytes32 resultHash, uint256 proposalTime);

    event ResultFinalized(bytes32 indexed eventId, uint8 result, address indexed proposer, uint256 finalizeTime);        address public admin;

    event ResultDisputed(bytes32 indexed eventId, address indexed challenger, string reason, uint256 challengeTime);

    event DisputeResolved(bytes32 indexed eventId, uint8 finalResult);    address public admin;    uint256 public constant DISPUTE_WINDOW = 1 days;

    

    modifier onlyAdmin() {    uint256 public disputeWindow = 1 days;    uint256 public constant MIN_JURY_VOTES = 3;

        require(msg.sender == admin, "Only admin");

        _;    uint256 public jurySize = 5;

    }

        uint256 public juryThreshold = 3; // Majority    /// @notice Proposed or finalized event result

    modifier eventExists(bytes32 eventId) {

        require(events[eventId].exists, "Event not found");        struct EventResult {

        _;

    }    enum ResultStatus {        bool exists;

    

    constructor() {        Proposed,        bool finalized;

        admin = msg.sender;

    }        Finalized,        uint8 result;              // 0 = NO, 1 = YES

    

    function createEvent(bytes32 eventId, string calldata description) external onlyAdmin {        Disputed,        uint256 proposedAt;

        require(!events[eventId].exists, "Event exists");

                Resolved        uint256 finalizedAt;

        Event storage e = events[eventId];

        e.exists = true;    }        bytes32 resultHash;

        e.status = ResultStatus.Proposed;

        e.proposalTime = block.timestamp;            address proposer;

        

        emit EventCreated(eventId, description, block.timestamp);    struct Event {        bytes32 aggregateDataHash;  // keccak256(aggregateValue, decryptors, ...)

    }

            bool exists;    }

    function proposeResult(bytes32 eventId, bytes32 resultHash) external eventExists(eventId) {

        Event storage e = events[eventId];        ResultStatus status;

        require(e.status == ResultStatus.Proposed, "Already proposed");

                uint8 result;                  // 0 = No, 1 = Yes    /// @notice Dispute record

        e.resultHash = resultHash;

        e.proposer = msg.sender;        uint256 proposalTime;    struct Dispute {

        

        emit ResultProposed(eventId, msg.sender, resultHash, block.timestamp);        uint256 finalizeTime;        bool active;

    }

            bytes32 resultHash;        uint256 initiatedAt;

    function finalizeResult(

        bytes32 eventId,        address proposer;        address initiator;

        uint8 result,

        bytes calldata decryptorSignatures,        uint256 totalStake;        string reason;

        bytes calldata proof

    ) external eventExists(eventId) {        mapping(address => uint256) yesStake;        uint256 juryVotesYes;

        Event storage e = events[eventId];

        require(e.status == ResultStatus.Proposed, "Not proposed");        mapping(address => uint256) noStake;        uint256 juryVotesNo;

        require(result <= 1, "Invalid result");

            }        mapping(address => bool) juryVoted;

        // TODO: Verify MPC signatures (3-of-5 threshold)

        // TODO: Verify resultHash matches        }

        

        e.result = result;    struct Dispute {

        e.status = ResultStatus.Finalized;

        e.finalizeTime = block.timestamp;        bool exists;    mapping(bytes32 => EventResult) public events;

        

        emit ResultFinalized(eventId, result, msg.sender, block.timestamp);        address challenger;    mapping(bytes32 => Dispute) public disputes;

    }

            string reason;    mapping(address => bool) public juryMembers;

    function disputeResult(

        bytes32 eventId,        uint256 challengeTime;

        string calldata reason

    ) external payable eventExists(eventId) {        uint8 juryResult;              // Will be set after vote    event ResultProposed(

        Event storage e = events[eventId];

        require(e.status == ResultStatus.Finalized, "Not finalized");        uint256 juryVotes;        bytes32 indexed eventId,

        require(block.timestamp <= e.finalizeTime + disputeWindow, "Window closed");

        require(msg.value > 0, "Must stake");    }        address indexed proposer,

        require(!disputes[eventId].exists, "Already disputed");

                    uint8 result,

        Dispute storage d = disputes[eventId];

        d.exists = true;    // ========== Storage ==========        bytes32 resultHash

        d.challenger = msg.sender;

        d.reason = reason;        );

        d.challengeTime = block.timestamp;

            mapping(bytes32 => Event) public events;

        e.status = ResultStatus.Disputed;

            mapping(bytes32 => Dispute) public disputes;    event ResultFinalized(

        emit ResultDisputed(eventId, msg.sender, reason, block.timestamp);

    }    mapping(bytes32 => mapping(address => bool)) public hasVoted; // jury votes        bytes32 indexed eventId,

    

    function getEventStatus(bytes32 eventId) external view returns (ResultStatus) {            uint8 result,

        return events[eventId].status;

    }    // ========== Events ==========        uint256 aggregateValue

    

    function getEventResult(bytes32 eventId) external view returns (uint8) {        );

        return events[eventId].result;

    }    event EventCreated(

}

        bytes32 indexed eventId,    event ResultDisputed(

        string description,        bytes32 indexed eventId,

        uint256 timestamp        address indexed challenger,

    );        string reason

        );

    event ResultProposed(

        bytes32 indexed eventId,    event JuryVoted(

        address indexed proposer,        bytes32 indexed eventId,

        bytes32 resultHash,        address indexed juror,

        uint256 proposalTime        uint8 vote  // 0 = no, 1 = yes

    );    );

    

    event ResultFinalized(    event DisputeResolved(

        bytes32 indexed eventId,        bytes32 indexed eventId,

        uint8 result,        uint8 juryResult

        address indexed proposer,    );

        uint256 finalizeTime

    );    modifier onlyAdmin() {

            require(msg.sender == admin, "FHEOracle: only admin");

    event ResultDisputed(        _;

        bytes32 indexed eventId,    }

        address indexed challenger,

        string reason,    modifier onlyJury() {

        uint256 challengeTime        require(juryMembers[msg.sender], "FHEOracle: only jury member");

    );        _;

        }

    event DisputeResolved(

        bytes32 indexed eventId,    constructor() {

        uint8 finalResult,        admin = msg.sender;

        address winner        // Add some mock jury members (for MVP)

    );        juryMembers[0x1111111111111111111111111111111111111111] = true;

            juryMembers[0x2222222222222222222222222222222222222222] = true;

    event PayoutClaimed(        juryMembers[0x3333333333333333333333333333333333333333] = true;

        bytes32 indexed eventId,    }

        address indexed claimant,

        uint256 amount    /**

    );     * @dev Propose a result for an event

         * @param eventId Event identifier

    // ========== Modifiers ==========     * @param result Result (0=NO, 1=YES)

         * @param resultHash keccak256 hash of aggregated computation

    modifier onlyAdmin() {     */

        require(msg.sender == admin, "Only admin");    function proposeResult(

        _;        bytes32 eventId,

    }        uint8 result,

            bytes32 resultHash

    modifier eventExists(bytes32 eventId) {    ) external {

        require(events[eventId].exists, "Event not found");        require(result == 0 || result == 1, "FHEOracle: invalid result");

        _;        

    }        EventResult storage e = events[eventId];

            require(!e.exists, "FHEOracle: result already proposed");

    modifier eventNotFinalized(bytes32 eventId) {

        require(        e.exists = true;

            events[eventId].status != ResultStatus.Finalized &&        e.result = result;

            events[eventId].status != ResultStatus.Resolved,        e.proposedAt = block.timestamp;

            "Event already finalized"        e.resultHash = resultHash;

        );        e.proposer = msg.sender;

        _;

    }        emit ResultProposed(eventId, msg.sender, result, resultHash);

        }

    // ========== Constructor ==========

        /**

    constructor() {     * @dev Finalize result after dispute window (admin/operator only)

        admin = msg.sender;     * @param eventId Event identifier

    }     * @param aggregateValue The plaintext aggregate value from FHE

         * @param decryptors Array of addresses that performed threshold decrypt

    // ========== Core Functions ==========     */

        function finalizeResult(

    /**        bytes32 eventId,

     * @dev Create a new prediction event        uint256 aggregateValue,

     */        address[] calldata decryptors

    function createEvent(    ) external onlyAdmin {

        bytes32 eventId,        EventResult storage e = events[eventId];

        string calldata description        require(e.exists, "FHEOracle: not proposed");

    ) external onlyAdmin {        require(!e.finalized, "FHEOracle: already finalized");

        require(!events[eventId].exists, "Event already exists");

                // Check dispute window passed

        Event storage e = events[eventId];        bool canFinalize = (block.timestamp >= e.proposedAt + DISPUTE_WINDOW) &&

        e.exists = true;                           !disputes[eventId].active;

        e.status = ResultStatus.Proposed;        

        e.proposalTime = block.timestamp;        require(canFinalize, "FHEOracle: dispute window open or active dispute");

        

        emit EventCreated(eventId, description, block.timestamp);        e.finalized = true;

    }        e.finalizedAt = block.timestamp;

            e.aggregateDataHash = keccak256(abi.encode(aggregateValue, decryptors));

    /**

     * @dev Propose an aggregation result (hash only)        emit ResultFinalized(eventId, e.result, aggregateValue);

     * Full result data is signed by FHE decryptors off-chain    }

     */

    function proposeResult(    /**

        bytes32 eventId,     * @dev Initiate dispute within window

        bytes32 resultHash     * @param eventId Event identifier

    ) external eventExists(eventId) {     * @param reason Dispute reason

        Event storage e = events[eventId];     */

        require(e.status == ResultStatus.Proposed, "Already proposed");    function disputeResult(

                bytes32 eventId,

        e.resultHash = resultHash;        string calldata reason

        e.proposer = msg.sender;    ) external payable {

                EventResult storage e = events[eventId];

        emit ResultProposed(eventId, msg.sender, resultHash, block.timestamp);        require(e.exists, "FHEOracle: not proposed");

    }        require(!e.finalized, "FHEOracle: already finalized");

    

    /**        bool windowOpen = block.timestamp <= e.proposedAt + DISPUTE_WINDOW;

     * @dev Finalize result with off-chain signatures from FHE decryptors        require(windowOpen, "FHEOracle: dispute window closed");

     * 

     * In production, verify that:        Dispute storage d = disputes[eventId];

     * - signatures are from authorized decryptors (3-of-5 threshold)        require(!d.active, "FHEOracle: dispute already active");

     * - signatures validate over (eventId, result, proof)

     * - resultHash matches keccak256(abi.encode(result, proof))        d.active = true;

     */        d.initiatedAt = block.timestamp;

    function finalizeResult(        d.initiator = msg.sender;

        bytes32 eventId,        d.reason = reason;

        uint8 result,

        bytes calldata decryptorSignatures,        emit ResultDisputed(eventId, msg.sender, reason);

        bytes calldata proof    }

    ) external eventExists(eventId) {

        Event storage e = events[eventId];    /**

        require(e.status == ResultStatus.Proposed, "Not in proposed state");     * @dev Jury member votes on disputed result

        require(result <= 1, "Result must be 0 or 1");     * @param eventId Event identifier

             * @param vote Vote (0=NO the result is correct, 1=YES the result is wrong)

        // TODO: Verify MPC signatures (3-of-5 threshold)     */

        // require(verifyMPCSignatures(eventId, result, proof, decryptorSignatures), "Invalid signatures");    function juryVote(bytes32 eventId, uint8 vote) external onlyJury {

                require(vote == 0 || vote == 1, "FHEOracle: invalid vote");

        // TODO: Verify resultHash matches

        // require(keccak256(abi.encode(result, proof)) == e.resultHash, "Hash mismatch");        Dispute storage d = disputes[eventId];

                require(d.active, "FHEOracle: no active dispute");

        e.result = result;        require(!d.juryVoted[msg.sender], "FHEOracle: already voted");

        e.status = ResultStatus.Finalized;

        e.finalizeTime = block.timestamp;        d.juryVoted[msg.sender] = true;

        

        emit ResultFinalized(eventId, result, msg.sender, block.timestamp);        if (vote == 0) {

    }            d.juryVotesNo++;

            } else {

    /**            d.juryVotesYes++;

     * @dev Challenge a finalized result during dispute window        }

     */

    function disputeResult(        emit JuryVoted(eventId, msg.sender, vote);

        bytes32 eventId,

        string calldata reason        // Auto-resolve if threshold reached (simple majority)

    ) external payable eventExists(eventId) {        uint256 totalVotes = d.juryVotesYes + d.juryVotesNo;

        Event storage e = events[eventId];        if (totalVotes >= MIN_JURY_VOTES) {

        require(e.status == ResultStatus.Finalized, "Not finalized");            _resolveDispute(eventId);

        require(        }

            block.timestamp <= e.finalizeTime + disputeWindow,    }

            "Dispute window closed"

        );    /**

        require(msg.value > 0, "Must stake to dispute");     * @dev Internal: resolve a dispute

        require(!disputes[eventId].exists, "Already disputed");     */

            function _resolveDispute(bytes32 eventId) internal {

        Dispute storage d = disputes[eventId];        Dispute storage d = disputes[eventId];

        d.exists = true;        EventResult storage e = events[eventId];

        d.challenger = msg.sender;

        d.reason = reason;        uint8 juryResult = d.juryVotesYes > d.juryVotesNo ? 1 : 0;

        d.challengeTime = block.timestamp;

                d.active = false;

        e.status = ResultStatus.Disputed;

                // If jury voted NO (0), result stands and can be finalized

        emit ResultDisputed(eventId, msg.sender, reason, block.timestamp);        // If jury voted YES (1), reverse the result and allow re-submission

    }        if (juryResult == 1) {

                e.result = e.result == 1 ? 0 : 1; // Toggle

    /**        }

     * @dev Jury vote on disputed result (simplified)

     * In production, implement proper jury selection and weighted voting        emit DisputeResolved(eventId, juryResult);

     */    }

    function juryVote(

        bytes32 eventId,    /**

        uint8 voteResult     * @dev Query a finalized result

    ) external eventExists(eventId) {     */

        Dispute storage d = disputes[eventId];    function getResult(bytes32 eventId)

        require(d.exists, "No dispute");        external

        require(!hasVoted[eventId][msg.sender], "Already voted");        view

        require(voteResult <= 1, "Vote must be 0 or 1");        returns (

                    bool exists,

        hasVoted[eventId][msg.sender] = true;            bool finalized,

        if (voteResult == 1) {            uint8 result,

            d.juryVotes++;            uint256 finalizedAt

        }        )

            {

        // Simple resolution: once threshold reached        EventResult storage e = events[eventId];

        if (d.juryVotes >= juryThreshold) {        return (e.exists, e.finalized, e.result, e.finalizedAt);

            d.juryResult = 1;    }

            _resolveDispute(eventId, 1);

        }    /**

    }     * @dev Check if dispute is active

         */

    /**    function isDisputeActive(bytes32 eventId) external view returns (bool) {

     * @dev Resolve dispute after jury voting        return disputes[eventId].active;

     */    }

    function _resolveDispute(bytes32 eventId, uint8 finalResult) internal {

        Event storage e = events[eventId];    /**

        require(e.status == ResultStatus.Disputed, "Not disputed");     * @dev Admin: add jury member (MVP)

             */

        // Update final result if jury overrode    function addJuryMember(address member) external onlyAdmin {

        if (finalResult != e.result) {        juryMembers[member] = true;

            e.result = finalResult;    }

        }

            /**

        e.status = ResultStatus.Resolved;     * @dev Admin: remove jury member (MVP)

             */

        emit DisputeResolved(    function removeJuryMember(address member) external onlyAdmin {

            eventId,        juryMembers[member] = false;

            e.result,    }

            disputes[eventId].challenger}

        );
    }
    
    /**
     * @dev Claim payout based on prediction result
     */
    function claimPayout(
        bytes32 eventId,
        bool predictedYes
    ) external eventExists(eventId) {
        Event storage e = events[eventId];
        require(
            e.status == ResultStatus.Finalized || e.status == ResultStatus.Resolved,
            "Event not settled"
        );
        
        uint256 stake = predictedYes ? e.yesStake[msg.sender] : e.noStake[msg.sender];
        require(stake > 0, "No stake found");
        
        bool won = (e.result == 1 && predictedYes) || (e.result == 0 && !predictedYes);
        require(won, "Prediction incorrect");
        
        // Simple payout: split total pot among winners (simplified)
        uint256 totalWinnerStake = (e.result == 1)
            ? _getTotalYesStake(eventId)
            : _getTotalNoStake(eventId);
        
        require(totalWinnerStake > 0, "No winners");
        
        uint256 payout = (stake * e.totalStake) / totalWinnerStake;
        
        // Clear stake to prevent re-entry
        if (predictedYes) {
            e.yesStake[msg.sender] = 0;
        } else {
            e.noStake[msg.sender] = 0;
        }
        
        // Transfer payout
        (bool success, ) = msg.sender.call{ value: payout }("");
        require(success, "Transfer failed");
        
        emit PayoutClaimed(eventId, msg.sender, payout);
    }
    
    // ========== Helper Functions ==========
    
    function _getTotalYesStake(bytes32 eventId) internal view returns (uint256) {
        // In production, iterate over all participants or maintain aggregate
        return 0; // Placeholder
    }
    
    function _getTotalNoStake(bytes32 eventId) internal view returns (uint256) {
        // In production, iterate over all participants or maintain aggregate
        return 0; // Placeholder
    }
    
    function getEventStatus(bytes32 eventId) external view returns (ResultStatus) {
        return events[eventId].status;
    }
    
    function getEventResult(bytes32 eventId) external view returns (uint8) {
        return events[eventId].result;
    }
}
