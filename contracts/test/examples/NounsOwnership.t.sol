// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {StorageOracle} from "src/examples/StorageOracle.sol";
import {NounsOwnership} from "src/examples/NounsOwnership.sol";
import {SuccinctGateway} from "src/SuccinctGateway.sol";

import {ISuccinctGatewayEvents, ISuccinctGatewayErrors} from "src/interfaces/ISuccinctGateway.sol";
import {MockSuccinctGateway} from "src/mocks/MockSuccinctGateway.sol";
import {Proxy} from "src/upgrades/Proxy.sol";

contract TestErrors is ISuccinctGatewayErrors {
    error InvalidL1BlockHash();
    error InvalidL1BlockNumber();
    error NotFromSuccinctGateway(address sender);
    error OutdatedBlockNumber(uint256 blockNumber, uint256 storedBlockNumber);
}

contract TestEvents is ISuccinctGatewayEvents {
    event SlotRequested(
        uint256 indexed blockNumber,
        bytes32 indexed blockHash,
        address indexed account,
        uint256 slot
    );
    event SlotUpdated(
        uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value
    );
}

contract NounsOwnershipTest is Test, TestErrors, TestEvents {
    /// @dev Fork a specific block so that L1_Block.hash() returns a consistent value, and thus, proof input is the same.
    uint256 internal constant BLOCK_NUMBER = 12345678;
    /// @dev  Nouns NFT contract: https://etherscan.io/address/0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03
    address internal constant NOUNS_ACCOUNT = 0x9C8fF314C9Bc7F6e59A9d9225Fb22946427eDC03;
    /// @dev Get slot key for noun40.eth's (0xa555d1Ee16780B2d414eD97f4f169c0740099615) NFT
    // 1. `mapping(uint256 => address) private _owners` slot is #3
    // 2. tokenId is to check 40
    uint256 internal constant OWNERS_SLOT = 3;
    uint256 internal constant NOUN_NUMBER = 40;
    address internal constant NOUN_OWNER = 0xa555d1Ee16780B2d414eD97f4f169c0740099615;
    uint256 internal constant SLOT = uint256(keccak256(abi.encode(NOUN_NUMBER, OWNERS_SLOT)));
    bytes32 internal constant FUNCTION_ID = keccak256("STORAGE");

    address internal timelock;
    address internal guardian;
    address internal gateway;
    address internal storageOracle;
    address internal nounsOwnership;

    bool internal skipTest;

    modifier onlyWithFork() {
        if (skipTest) return;
        _;
    }

    function setUp() public {
        try vm.envString("RPC_420") returns (string memory RPC_420) {
            vm.createSelectFork(RPC_420, BLOCK_NUMBER);
        } catch {
            console.log("RPC_420 not set, skipping test");
            skipTest = true;
        }

        timelock = makeAddr("timelock");
        guardian = makeAddr("guardian");
        gateway = address(new MockSuccinctGateway());

        // Deploy StorageOracle
        address storageOracleImpl = address(new StorageOracle());
        storageOracle = address(new Proxy(storageOracleImpl, ""));
        StorageOracle(storageOracle).initialize(address(gateway), FUNCTION_ID, timelock, guardian);

        // Deploy NounsOwnership
        nounsOwnership = address(new NounsOwnership(storageOracle));
    }

    function test_OwnerOf_WithLoadFixture() public onlyWithFork {
        // Set input and output from fixture
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/test/fixtures/nouns-fixture.json");
        MockSuccinctGateway(gateway).loadFixture(path);

        // Request slot
        NounsOwnership(nounsOwnership).claimOwner(NOUN_NUMBER);

        address owner = NounsOwnership(nounsOwnership).ownerOf(NOUN_NUMBER);
        assertEq(NOUN_OWNER, owner);
    }

    function test_OwnerOf_WithLoadInputOutput() public onlyWithFork {
        // Set input and output from inline
        MockSuccinctGateway(gateway).loadInputOutput(
            hex"b99c7a251e3880d3560ff23412f4b880c196c252a791a4667694447892c051a70000000000000000000000009c8ff314c9bc7f6e59a9d9225fb22946427edc03f787d5ff306ee7ea1d7b35b5cacd5a837646921c113945dbc3a3b6329ce40033",
            hex"000000000000000000000000a555d1Ee16780B2d414eD97f4f169c0740099615"
        );

        // Request slot
        NounsOwnership(nounsOwnership).claimOwner(NOUN_NUMBER);

        address owner = NounsOwnership(nounsOwnership).ownerOf(NOUN_NUMBER);
        assertEq(NOUN_OWNER, owner);
    }
}
