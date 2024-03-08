// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {ISuccinctGateway} from "src/interfaces/ISuccinctGateway.sol";
import {Initializable} from "@openzeppelin-upgradeable/contracts/proxy/utils/Initializable.sol";

/// @title EVMCallback
contract EVMCallback is Initializable {
    /// @notice The SuccinctGateway contract address.
    address public GATEWAY;

    /// @notice The identifier for the storage verifier.
    bytes32 public FUNCTION_ID;

    uint256 public nonce;

    event Requested(bytes32 requestHash);
    event Updated(bytes output, bytes context);

    error InvalidL1BlockHash();
    error InvalidL1BlockNumber();
    error NotFromSuccinctGateway(address sender);
    error OutdatedBlockNumber(uint256 blockNumber, uint256 storedBlockNumber);

    /// @param _gateway The SuccinctGateway address.
    /// @param _functionId The functionId for the storage verifier.
    function initialize(address _gateway, bytes32 _functionId) external initializer {
        GATEWAY = _gateway;
        FUNCTION_ID = _functionId;
    }

    function request() external payable returns (bytes32 requestHash) {
        requestHash = ISuccinctGateway(GATEWAY).requestCallback{value: msg.value}(
            FUNCTION_ID, hex"0205", _toBytes(nonce), EVMCallback.handle.selector, 1_000_000
        );

        emit Requested(requestHash);

        nonce++;
    }

    function handle(bytes memory _output, bytes memory _context) external {
        if (msg.sender != GATEWAY || !ISuccinctGateway(GATEWAY).isCallback()) {
            revert NotFromSuccinctGateway(msg.sender);
        }

        emit Updated(_output, _context);
    }

    function _toBytes(uint256 x) internal pure returns (bytes memory b) {
        b = new bytes(32);
        assembly {
            mstore(add(b, 32), x)
        }
    }
}
