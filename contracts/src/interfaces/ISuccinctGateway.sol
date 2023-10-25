// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface ISuccinctGatewayEvents {
    event RequestCallback(
        uint32 indexed nonce,
        bytes32 indexed functionId,
        bytes input,
        bytes context,
        address callbackAddress,
        bytes4 callbackSelector,
        uint32 callbackGasLimit,
        uint256 feeAmount
    );
    event RequestCall(
        bytes32 indexed functionId,
        bytes input,
        address entryAddress,
        bytes entryCalldata,
        uint32 entryGasLimit,
        address sender,
        uint256 feeAmount
    );
    event RequestFulfilled(
        uint32 indexed nonce, bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash
    );
    event Call(bytes32 indexed functionId, bytes32 inputHash, bytes32 outputHash);
}

interface ISuccinctGatewayErrors {
    error InvalidRequest(uint32 nonce, bytes32 expectedRequestHash, bytes32 requestHash);
    error CallbackFailed(bytes4 callbackSelector, bytes output, bytes context);
    error InvalidCall(bytes32 functionId, bytes input);
    error CallFailed(address callbackAddress, bytes callbackData);
    error InvalidProof(address verifier, bytes32 inputHash, bytes32 outputHash, bytes proof);
}

interface ISuccinctGateway is ISuccinctGatewayEvents, ISuccinctGatewayErrors {
    function requestCallback(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable returns (bytes32);

    function requestCall(
        bytes32 _functionId,
        bytes memory _input,
        address _entryAddress,
        bytes memory _entryData,
        uint32 _entryGasLimit
    ) external payable;

    function verifiedCall(bytes32 _functionId, bytes memory _input)
        external
        view
        returns (bytes memory);

    function isCallback() external view returns (bool);
}
