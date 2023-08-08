// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "script/misc/Base.s.sol";
import {Timelock} from "src/upgrade/Timelock.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

// Flow here is EOA(msg.sender) -> Safe -> Timelock -> Proxy.
// We (the EOA) sign a Safe transaction, and this transaction is a call to Timelock's schedule(),
// with the Target being the Proxy and the Data being the Proxy's upgradeTo() function:
//
// 		TIMELOCK.schedule(
// 			PROXY, 0, abi.encodeWithSelector(IProxy.upgradeTo.selector, IMPL), 0, CREATE2_SALT, MINIMUM_DELAY
// 		);
//
contract UpgradeSignSchedule is BaseScript {
    function run(address PROXY, address IMPL) external returns (address signer, bytes memory signature) {
        // Check inputs
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        console.log("PROXY: %s", Strings.toHexString(uint160(PROXY), 20));
        console.log("IMPL: %s", Strings.toHexString(uint160(IMPL), 20));

        bytes memory scheduleBatchData;
        {
            bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");

            // Generate the scheduleBatch call
            address[] memory targets = new address[](1);
            targets[0] = PROXY;

            uint256[] memory values = new uint256[](1);
            values[0] = 0;

            bytes[] memory payloads = new bytes[](1);
            payloads[0] = abi.encodeWithSelector(IProxy.upgradeTo.selector, IMPL);

            bytes32 id = ITimelock(TIMELOCK).hashOperationBatch(targets, values, payloads, 0, CREATE2_SALT);
            if (ITimelock(TIMELOCK).isOperation(id)) {
                revert("operation already exists in Timelock, change CREATE2_SALT to schedule a new one");
            }

            scheduleBatchData = abi.encodeWithSelector(
                ITimelock.scheduleBatch.selector,
                targets,
                values,
                payloads,
                0,
                CREATE2_SALT,
                ITimelock(TIMELOCK).getMinDelay()
            );
        }

        {
            // Use the scheduleBatch call as for the Safe TX to sign.
            uint256 nonce = ISafe(GUARDIAN).nonce();
            bytes memory txEncoded = ISafe(GUARDIAN).encodeTransactionData({
                to: TIMELOCK,
                value: 0,
                data: scheduleBatchData,
                operation: Enum.Operation.Call,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                _nonce: nonce
            });
            bytes32 txHash = keccak256(txEncoded);
            signer = getSigner();
            signature = sign(Strings.toHexString(uint256(txHash)));

            // Verify that signature is valid and is for an owner of the Safe.
            ISafe(GUARDIAN).checkNSignatures(txHash, txEncoded, signature, 1);
        }

        console.log("signature for TX returned successfully");
    }
}

// After enough signatures have been collected, a call to Safe.execTransaction(..., signatures) is
// made which schedules the call on the Timelock.
contract UpgradeSendSchedule is BaseScript {
    function run(address PROXY, address IMPL, bytes memory _signatures) external broadcaster returns (bool success) {
        // Check inputs
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        console.log("PROXY: %s", Strings.toHexString(uint160(PROXY), 20));
        console.log("IMPL: %s", Strings.toHexString(uint160(IMPL), 20));

        bytes memory scheduleBatchData;
        {
            bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");

            // Generate the scheduleBatch call
            address[] memory targets = new address[](1);
            targets[0] = PROXY;

            uint256[] memory values = new uint256[](1);
            values[0] = 0;

            bytes[] memory payloads = new bytes[](1);
            payloads[0] = abi.encodeWithSelector(IProxy.upgradeTo.selector, IMPL);

            bytes32 id = ITimelock(TIMELOCK).hashOperationBatch(targets, values, payloads, 0, CREATE2_SALT);
            if (ITimelock(TIMELOCK).isOperation(id)) {
                revert("operation already exists in Timelock, change CREATE2_SALT to schedule a new one");
            }

            scheduleBatchData = abi.encodeWithSelector(
                ITimelock.scheduleBatch.selector,
                targets,
                values,
                payloads,
                0,
                CREATE2_SALT,
                ITimelock(TIMELOCK).getMinDelay()
            );
        }

        {
            if (ISafe(GUARDIAN).getThreshold() * 65 > _signatures.length) {
                console.log(
                    "not enough signatures, need %d have %d", ISafe(GUARDIAN).getThreshold(), _signatures.length / 65
                );
                return false;
            }

            // Use the scheduleBatch call as for the Safe TX to execute with the signatues.
            success = ISafe(GUARDIAN).execTransaction({
                to: TIMELOCK,
                value: 0,
                data: scheduleBatchData,
                operation: Enum.Operation.Call,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                signatures: _signatures
            });
        }
        console.log("TX executed successfully");
    }
}

// After MINIMUM_DELAY has passed, the call to Timelock.execute() can be made.
contract UpgradeSignExecute is BaseScript {
    function run(address PROXY, address IMPL) external returns (address signer, bytes memory signature) {
        // Check inputs
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        console.log("PROXY: %s", Strings.toHexString(uint160(PROXY), 20));
        console.log("IMPL: %s", Strings.toHexString(uint160(IMPL), 20));

        bytes memory executeBatchData;
        {
            bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");

            // Generate the executeBatch call
            address[] memory targets = new address[](1);
            targets[0] = PROXY;

            uint256[] memory values = new uint256[](1);
            values[0] = 0;

            bytes[] memory payloads = new bytes[](1);
            payloads[0] = abi.encodeWithSelector(IProxy.upgradeTo.selector, IMPL);

            bytes32 id = ITimelock(TIMELOCK).hashOperationBatch(targets, values, payloads, 0, CREATE2_SALT);
            if (ITimelock(TIMELOCK).isOperationDone(id)) {
                console.log("operation already executed in Timelock");
                return (address(0), "");
            }

            executeBatchData = abi.encodeWithSelector(
                ITimelock.executeBatch.selector,
                targets,
                values,
                payloads,
                0,
                CREATE2_SALT,
                ITimelock(TIMELOCK).getMinDelay()
            );
        }

        {
            // Use the executeBatch call as for the Safe TX to sign.
            uint256 nonce = ISafe(GUARDIAN).nonce();
            bytes memory txEncoded = ISafe(GUARDIAN).encodeTransactionData({
                to: TIMELOCK,
                value: 0,
                data: executeBatchData,
                operation: Enum.Operation.Call,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                _nonce: nonce
            });
            bytes32 txHash = keccak256(txEncoded);
            signer = getSigner();
            signature = sign(Strings.toHexString(uint256(txHash)));

            // Verify that signature is valid and is for an owner of the Safe.
            ISafe(GUARDIAN).checkNSignatures(txHash, txEncoded, signature, 1);
        }
        console.log("signature for TX returned successfully");
    }
}

contract UpgradeSendExecute is BaseScript {
    function run(address PROXY, address IMPL, bytes memory _signatures) external broadcaster returns (bool success) {
        // Check inputs
        address TIMELOCK = envAddress("TIMELOCK", block.chainid);
        address GUARDIAN = envAddress("GUARDIAN", block.chainid);
        console.log("PROXY: %s", Strings.toHexString(uint160(PROXY), 20));
        console.log("IMPL: %s", Strings.toHexString(uint160(IMPL), 20));

        bytes memory executeBatchData;
        {
            bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");

            // Generate the executeBatch call
            address[] memory targets = new address[](1);
            targets[0] = PROXY;

            uint256[] memory values = new uint256[](1);
            values[0] = 0;

            bytes[] memory payloads = new bytes[](1);
            payloads[0] = abi.encodeWithSelector(IProxy.upgradeTo.selector, IMPL);

            bytes32 id = ITimelock(TIMELOCK).hashOperationBatch(targets, values, payloads, 0, CREATE2_SALT);
            if (ITimelock(TIMELOCK).isOperationDone(id)) {
                console.log("operation already executed in Timelock");
                return true;
            }

            executeBatchData = abi.encodeWithSelector(
                ITimelock.executeBatch.selector,
                targets,
                values,
                payloads,
                0,
                CREATE2_SALT,
                ITimelock(TIMELOCK).getMinDelay()
            );
        }

        {
            if (ISafe(GUARDIAN).getThreshold() * 65 > _signatures.length) {
                console.log(
                    "not enough signatures, need %d have %d", ISafe(GUARDIAN).getThreshold(), _signatures.length / 65
                );
                return false;
            }

            // Use the executeBatch call as for the Safe TX to execute with the signatues.
            success = ISafe(GUARDIAN).execTransaction({
                to: TIMELOCK,
                value: 0,
                data: executeBatchData,
                operation: Enum.Operation.Call,
                safeTxGas: 0,
                baseGas: 0,
                gasPrice: 0,
                gasToken: address(0),
                refundReceiver: payable(address(0)),
                signatures: _signatures
            });
        }
        console.log("TX executed successfully");
    }
}

interface IProxy {
    function upgradeTo(address newImplementation) external;
}

/// @notice https://github.com/OpenZeppelin/openzeppelin-contracts/blob/f347b410cf6aeeaaf5197e1fece139c793c03b2b/contracts/governance/TimelockController.sol
interface ITimelock {
    function getMinDelay() external view returns (uint256);

    function isOperation(bytes32 id) external view returns (bool);

    function isOperationDone(bytes32 id) external view returns (bool);

    function hashOperationBatch(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata payloads,
        bytes32 predecessor,
        bytes32 salt
    ) external pure returns (bytes32);

    /// @dev Caller must be PROPOSER_ADDRESS.
    function scheduleBatch(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata payloads,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) external;

    /// @dev Caller must be EXECUTOR_ADDRESS.
    function executeBatch(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata payloads,
        bytes32 predecessor,
        bytes32 salt
    ) external payable;
}

abstract contract Enum {
    enum Operation {
        Call,
        DelegateCall
    }
}

interface ISafe {
    event AddedOwner(address owner);
    event ApproveHash(bytes32 indexed approvedHash, address indexed owner);
    event ChangedFallbackHandler(address handler);
    event ChangedGuard(address guard);
    event ChangedThreshold(uint256 threshold);
    event DisabledModule(address module);
    event EnabledModule(address module);
    event ExecutionFailure(bytes32 txHash, uint256 payment);
    event ExecutionFromModuleFailure(address indexed module);
    event ExecutionFromModuleSuccess(address indexed module);
    event ExecutionSuccess(bytes32 txHash, uint256 payment);
    event RemovedOwner(address owner);
    event SafeReceived(address indexed sender, uint256 value);
    event SafeSetup(
        address indexed initiator, address[] owners, uint256 threshold, address initializer, address fallbackHandler
    );
    event SignMsg(bytes32 indexed msgHash);

    function VERSION() external view returns (string memory);
    function addOwnerWithThreshold(address owner, uint256 _threshold) external;
    function approveHash(bytes32 hashToApprove) external;
    function approvedHashes(address, bytes32) external view returns (uint256);
    function changeThreshold(uint256 _threshold) external;
    function checkNSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures, uint256 requiredSignatures)
        external
        view;
    function checkSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures) external view;
    function disableModule(address prevModule, address module) external;
    function domainSeparator() external view returns (bytes32);
    function enableModule(address module) external;
    function encodeTransactionData(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        uint256 _nonce
    ) external view returns (bytes memory);
    function execTransaction(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        bytes memory signatures
    ) external payable returns (bool success);
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation)
        external
        returns (bool success);
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation)
        external
        returns (bool success, bytes memory returnData);
    function getChainId() external view returns (uint256);
    function getModulesPaginated(address start, uint256 pageSize)
        external
        view
        returns (address[] memory array, address next);
    function getOwners() external view returns (address[] memory);
    function getStorageAt(uint256 offset, uint256 length) external view returns (bytes memory);
    function getThreshold() external view returns (uint256);
    function getTransactionHash(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        uint256 _nonce
    ) external view returns (bytes32);
    function isModuleEnabled(address module) external view returns (bool);
    function isOwner(address owner) external view returns (bool);
    function nonce() external view returns (uint256);
    function removeOwner(address prevOwner, address owner, uint256 _threshold) external;
    function requiredTxGas(address to, uint256 value, bytes memory data, Enum.Operation operation)
        external
        returns (uint256);
    function setFallbackHandler(address handler) external;
    function setGuard(address guard) external;
    function setup(
        address[] memory _owners,
        uint256 _threshold,
        address to,
        bytes memory data,
        address fallbackHandler,
        address paymentToken,
        uint256 payment,
        address paymentReceiver
    ) external;
    function signedMessages(bytes32) external view returns (uint256);
    function simulateAndRevert(address targetContract, bytes memory calldataPayload) external;
    function swapOwner(address prevOwner, address oldOwner, address newOwner) external;
}
