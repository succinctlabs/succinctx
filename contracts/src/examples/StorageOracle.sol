// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {ISuccinctGateway} from "src/interfaces/ISuccinctGateway.sol";
import {TimelockedUpgradeable} from "src/upgrades/TimelockedUpgradeable.sol";

/// @dev https://community.optimism.io/docs/protocol/protocol-2.0/#l1block
interface L1BlockPrecompile {
    function number() external view returns (uint64);
    function timestamp() external view returns (uint64);
    function hash() external view returns (bytes32);
}

/// @title StorageOracle
/// @notice Stores slot values of L1 accounts.
contract StorageOracle is TimelockedUpgradeable {
    /// @notice Represents a storage slot value on L1, with the block number it was retrieved from.
    struct StorageSlot {
        bytes32 value;
        uint256 blockNumber;
    }

    /// @dev https://community.optimism.io/docs/developers/build/differences/#opcode-differences
    L1BlockPrecompile public constant L1_BLOCK =
        L1BlockPrecompile(0x4200000000000000000000000000000000000015);
    uint32 public constant DEFAULT_GAS_LIMIT = 1000000;

    /// @notice The SuccinctGateway contract address.
    address public GATEWAY;

    /// @notice The identifier for the storage verifier.
    bytes32 public FUNCTION_ID;

    /// @notice Mapping for L1 account -> storage slot number -> storage slot value.
    mapping(address => mapping(uint256 => StorageSlot)) public slots;

    event SlotRequested(
        uint256 indexed blockNumber,
        bytes32 indexed blockHash,
        address indexed account,
        uint256 slot
    );
    event SlotUpdated(
        uint256 indexed blockNumber, address indexed account, uint256 slot, bytes32 value
    );

    error InvalidL1BlockHash();
    error InvalidL1BlockNumber();
    error NotFromSuccinctGateway(address sender);
    error OutdatedBlockNumber(uint256 blockNumber, uint256 storedBlockNumber);

    /// @param _gateway The SuccinctGateway address.
    /// @param _functionId The functionId for the storage verifier.
    function initialize(address _gateway, bytes32 _functionId, address _timelock, address _guardian)
        external
        initializer
    {
        GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
        __TimelockedUpgradeable_init(_timelock, _guardian);
    }

    /// @notice Request a storage slot value for a given account on L1.
    function requestStorageSlot(address _account, uint256 _slot)
        external
        payable
        returns (bytes32 requestHash)
    {
        bytes32 blockHash = L1_BLOCK.hash();
        if (blockHash == bytes32(0)) {
            revert InvalidL1BlockHash();
        }
        uint256 blockNumber = L1_BLOCK.number();
        if (blockNumber == 0) {
            revert InvalidL1BlockNumber();
        }

        bytes memory input = abi.encode(blockHash, _account, _slot);
        bytes memory context = abi.encode(blockNumber, _account, _slot);
        requestHash = ISuccinctGateway(GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, input, context, StorageOracle.handleStorageSlot.selector, DEFAULT_GAS_LIMIT
        );

        emit SlotRequested(blockNumber, blockHash, _account, _slot);
    }

    /// @dev Callback function to recieve the storage slot value from the SuccinctGateway. If for existing slot, MUST update
    ///      for a more recent blockNumber.
    function handleStorageSlot(bytes memory _output, bytes memory _context) external {
        if (msg.sender != GATEWAY || !ISuccinctGateway(GATEWAY).isCallback()) {
            revert NotFromSuccinctGateway(msg.sender);
        }

        bytes32 slotValue = abi.decode(_output, (bytes32));
        (uint256 blockNumber, address account, uint256 slot) =
            abi.decode(_context, (uint256, address, uint256));
        if (blockNumber <= slots[account][slot].blockNumber) {
            revert OutdatedBlockNumber(blockNumber, slots[account][slot].blockNumber);
        }

        slots[account][slot] = StorageSlot(slotValue, blockNumber);

        emit SlotUpdated(blockNumber, account, slot, slotValue);
    }
}
