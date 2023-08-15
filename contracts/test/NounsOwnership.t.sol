// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {StorageOracle} from "../src/examples/storage/StorageOracle.sol";
import {NounsOwnership} from "../src/examples/storage/NounsOwnership.sol";
import {FunctionGateway} from "../src/FunctionGateway.sol";

import {IFunctionGatewayEvents, IFunctionGatewayErrors} from "../src/interfaces/IFunctionGateway.sol";
import {MockFunctionGateway} from "../src/mocks/MockFunctionGateway.sol";
import {Proxy} from "src/upgrades/Proxy.sol";

contract TestError is IFunctionGatewayErrors {
    error InvalidL1BlockHash();
    error InvalidL1BlockNumber();
    error NotFromFunctionGateway(address sender);
    error OutdatedBlockNumber(uint256 blockNumber, uint256 storedBlockNumber);
}

contract TestEvents is IFunctionGatewayEvents {
    event SlotRequested(uint256 indexed blockNumber, bytes32 indexed blockHash, address indexed account, uint256 slot);
    event SlotUpdated(uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value);
}

contract NounsOwnershipTest is Test, TestError, TestEvents {
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
        gateway = address(new MockFunctionGateway());

        // Deploy StorageOracle
        address storageOracleImpl = address(new StorageOracle());
        storageOracle = address(new Proxy(storageOracleImpl, ""));
        StorageOracle(storageOracle).initialize(address(gateway), FUNCTION_ID, timelock, guardian);

        nounsOwnership = address(new NounsOwnership(storageOracle));
    }

    function test_ClaimOwnership() public onlyWithFork {
        bytes32 requestId = NounsOwnership(nounsOwnership).claimOwnership(NOUN_NUMBER);

        (bytes32 functionId,,,, address callbackAddress,, bool proofFulfilled, bool callbackFulfilled) =
            FunctionGateway(gateway).requests(requestId);
        assertEq(FUNCTION_ID, functionId);
        assertEq(storageOracle, callbackAddress);
        assertEq(false, proofFulfilled);
        assertEq(false, callbackFulfilled);
    }

    function test_OwnerOf() public onlyWithFork {
        bytes32 requestId = NounsOwnership(nounsOwnership).claimOwnership(NOUN_NUMBER);

        bytes memory output = abi.encode(NOUN_OWNER);
        bytes memory context = abi.encode(BLOCK_NUMBER, NOUNS_ACCOUNT, SLOT);
        MockFunctionGateway(gateway).callback(requestId, output, context);

        address owner = NounsOwnership(nounsOwnership).ownerOf(NOUN_NUMBER);
        assertEq(NOUN_OWNER, owner);
    }

    function test_LastUpdateBlock() public onlyWithFork {
        bytes32 requestId = NounsOwnership(nounsOwnership).claimOwnership(NOUN_NUMBER);

        bytes memory output = abi.encode(NOUN_OWNER);
        bytes memory context = abi.encode(BLOCK_NUMBER, NOUNS_ACCOUNT, SLOT);
        MockFunctionGateway(gateway).callback(requestId, output, context);

        uint256 blockNumber = NounsOwnership(nounsOwnership).lastUpdatedBlock(NOUN_NUMBER);
        assertEq(BLOCK_NUMBER, blockNumber);
    }

    function test_OwnerOf_WithFixture() public onlyWithFork {
        bytes memory context = abi.encode(BLOCK_NUMBER, NOUNS_ACCOUNT, SLOT);

        // Use input and output from fixture
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/test/fixtures/nouns-fixture.json");
        MockFunctionGateway(gateway).loadFixture(path);
        NounsOwnership(nounsOwnership).claimOwnership(NOUN_NUMBER);

        address owner = NounsOwnership(nounsOwnership).ownerOf(NOUN_NUMBER);
        assertEq(NOUN_OWNER, owner);
    }
}

// Nouns NFT contract: https://etherscan.io/address/0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03
// Storage Layout:
// | Name               | Type                                                                          | Slot | Offset | Bytes | Contract                            |
// |--------------------|-------------------------------------------------------------------------------|------|--------|-------|-------------------------------------|
// | _owner             | address                                                                       | 0    | 0      | 20    | contracts/NounsToken.sol:NounsToken |
// | _name              | string                                                                        | 1    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _symbol            | string                                                                        | 2    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _owners            | mapping(uint256 => address)                                                   | 3    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _balances          | mapping(address => uint256)                                                   | 4    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _tokenApprovals    | mapping(uint256 => address)                                                   | 5    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _operatorApprovals | mapping(address => mapping(address => bool))                                  | 6    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _ownedTokens       | mapping(address => mapping(uint256 => uint256))                               | 7    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _ownedTokensIndex  | mapping(uint256 => uint256)                                                   | 8    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _allTokens         | uint256[]                                                                     | 9    | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _allTokensIndex    | mapping(uint256 => uint256)                                                   | 10   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _delegates         | mapping(address => address)                                                   | 11   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | checkpoints        | mapping(address => mapping(uint32 => struct ERC721Checkpointable.Checkpoint)) | 12   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | numCheckpoints     | mapping(address => uint32)                                                    | 13   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | nonces             | mapping(address => uint256)                                                   | 14   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | noundersDAO        | address                                                                       | 15   | 0      | 20    | contracts/NounsToken.sol:NounsToken |
// | minter             | address                                                                       | 16   | 0      | 20    | contracts/NounsToken.sol:NounsToken |
// | descriptor         | contract INounsDescriptorMinimal                                              | 17   | 0      | 20    | contracts/NounsToken.sol:NounsToken |
// | seeder             | contract INounsSeeder                                                         | 18   | 0      | 20    | contracts/NounsToken.sol:NounsToken |
// | isMinterLocked     | bool                                                                          | 18   | 20     | 1     | contracts/NounsToken.sol:NounsToken |
// | isDescriptorLocked | bool                                                                          | 18   | 21     | 1     | contracts/NounsToken.sol:NounsToken |
// | isSeederLocked     | bool                                                                          | 18   | 22     | 1     | contracts/NounsToken.sol:NounsToken |
// | seeds              | mapping(uint256 => struct INounsSeeder.Seed)                                  | 19   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _currentNounId     | uint256                                                                       | 20   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
// | _contractURIHash   | string                                                                        | 21   | 0      | 32    | contracts/NounsToken.sol:NounsToken |
