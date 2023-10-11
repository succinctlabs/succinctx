// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface IFunctionGatewayEvents {
    event Request(
        uint32 indexed nonce,
        bytes32 indexed functionId,
        bytes input,
        bytes context,
        bytes4 callbackSelector,
        uint32 callbackGasLimit
    );
    event RequestFulfilled(
        uint32 indexed nonce,
        bytes32 indexed functionId,
        bytes32 inputHash,
        bytes32 outputHash
    );
    event Call(
        bytes32 indexed functionId,
        bytes32 inputHash,
        bytes32 outputHash
    );
}

interface IFunctionGatewayErrors {
    error InvalidRequest(
        uint32 nonce,
        bytes32 expectedRequestHash,
        bytes32 requestHash
    );
    error CallbackFailed(bytes4 callbackSelector, bytes output, bytes context);
    error InvalidCall(bytes32 functionId, bytes input);
    error CallFailed(address callbackAddress, bytes callbackData);
    error InvalidProof(
        address verifier,
        bytes32 inputHash,
        bytes32 outputHash,
        bytes proof
    );
}

interface IFunctionGateway is IFunctionGatewayEvents, IFunctionGatewayErrors {
    function zkRequest(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable returns (bytes32);

    function zkCall(
        bytes32 _functionId,
        bytes memory _input
    ) external view returns (bool, bytes memory);
}
