// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import {VmSafe} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IFunctionGateway} from "../interfaces/IFunctionGateway.sol";

contract MockFunctionGateway is IFunctionGateway {
    VmSafe private constant vm = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));
    uint32 public nonce;
    bytes32 public verifiedFunctionId;
    bytes32 public verifiedInputHash;
    bytes public verifiedOutput;
    bool public isCallback;
    mapping(uint32 => bytes32) public requests;
    mapping(bytes32 => bytes) public outputs;

    function loadFixture(string memory _path) external {
        string memory json;

        try vm.readFile(_path) returns (string memory data) {
            json = data;
        } catch {
            revert("MockFunctionGateway: fixture not found");
        }

        bytes memory input = stdJson.readBytes(json, "$.input");
        bytes memory output = stdJson.readBytes(json, "$.output");
        bytes32 inputHash = sha256(input);
        outputs[inputHash] = output;
    }

    function requestCallback(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) public payable returns (bytes32) {
        bytes32 inputHash = sha256(_input);
        bytes32 contextHash = keccak256(_context);
        address callbackAddress = msg.sender;
        bytes32 requestHash = _requestHash(
            nonce,
            _functionId,
            inputHash,
            contextHash,
            callbackAddress,
            _callbackSelector,
            _callbackGasLimit
        );

        requests[nonce] = requestHash;
        emit RequestCallback(
            nonce,
            _functionId,
            _input,
            _context,
            callbackAddress,
            _callbackSelector,
            _callbackGasLimit,
            msg.value
        );
        nonce++;

        // If fixture has been pre-loaded, automatically fulfill and callback.
        bytes memory output = outputs[inputHash];
        if (output.length > 0) {
            bytes32 outputHash = sha256(output);

            isCallback = true;
            (bool status,) =
                msg.sender.call(abi.encodeWithSelector(_callbackSelector, output, _context));
            isCallback = false;
            if (!status) {
                revert CallbackFailed(_callbackSelector, output, _context);
            }

            emit RequestFulfilled(nonce, _functionId, inputHash, outputHash);
        }

        return requestHash;
    }

    /// @dev This function only exists to implement interface. Use requestCall with your target
    ///      callbackAddress and callbackData instead.
    function requestCall(
        bytes32 _functionId,
        bytes memory _input,
        address _entryAddress,
        bytes memory _entryCalldata,
        uint32 _entryGasLimit
    ) external payable {
        requestCall(
            _functionId,
            _input,
            _entryAddress,
            _entryCalldata,
            _entryGasLimit,
            _entryAddress,
            _entryCalldata
        );
    }

    function requestCall(
        bytes32 _functionId,
        bytes memory _input,
        address _entryAddress,
        bytes memory _entryCalldata,
        uint32 _entryGasLimit,
        address _callbackAddress,
        bytes memory _callbackData
    ) public payable {
        emit RequestCall(
            _functionId,
            _input,
            _entryAddress,
            _entryCalldata,
            _entryGasLimit,
            msg.sender,
            msg.value
        );

        // If fixture has been pre-loaded, automatically fulfill and callback.
        bytes32 inputHash = sha256(_input);
        bytes memory output = outputs[inputHash];
        if (output.length > 0) {
            bytes32 outputHash = sha256(output);

            verifiedFunctionId = _functionId;
            verifiedInputHash = inputHash;
            verifiedOutput = output;
            (bool status,) = _callbackAddress.call(_callbackData);
            if (!status) {
                revert CallFailed(_callbackAddress, _callbackData);
            }
            delete verifiedFunctionId;
            delete verifiedInputHash;
            delete verifiedOutput;

            emit Call(_functionId, inputHash, outputHash);
        }
    }

    function verifiedCall(bytes32 _functionId, bytes memory _input)
        external
        view
        returns (bytes memory)
    {
        bytes32 inputHash = sha256(_input);
        if (verifiedFunctionId == _functionId && verifiedInputHash == inputHash) {
            return verifiedOutput;
        } else {
            revert InvalidCall(_functionId, _input);
        }
    }

    function _requestHash(
        uint32 _nonce,
        bytes32 _functionId,
        bytes32 _inputHash,
        bytes32 _contextHash,
        address _callbackAddress,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encodePacked(
                _nonce,
                _functionId,
                _inputHash,
                _contextHash,
                _callbackAddress,
                _callbackSelector,
                _callbackGasLimit
            )
        );
    }
}
