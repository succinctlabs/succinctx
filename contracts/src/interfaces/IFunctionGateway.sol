// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

struct VerifiedCall {
    bytes32 functionId;
    bytes32 inputHash;
    bytes32 outputHash;
    bytes input;
    bytes output;
}

struct FunctionRequest {
    bytes32 functionId;
    bytes32 inputHash;
    bytes32 outputHash;
    bytes32 contextHash;
    address callbackAddress;
    bytes4 callbackSelector;
    bool proofFulfilled;
    bool callbackFulfilled;
}

interface IFunctionGatewayEvents {
    event ProofRequested(
        uint256 indexed nonce,
        bytes32 indexed functionId,
        bytes32 requestId,
        bytes inputs,
        bytes context,
        uint256 gasLimit,
        uint256 feeAmount
    );
    event ProofFulfilled(bytes32 requestId, bytes32 outputHash, bytes proof);
    event ProofBatchFulfilled(
        bytes32[] requestIds,
        bytes aggregateProof,
        bytes32 inputsRoot,
        bytes32[] outputHashes,
        bytes32 outputsRoot,
        bytes32 verificationKeyRoot
    );
    event CallbackFulfilled(bytes32 requestId, bytes output, bytes context);
    event ScalarUpdated(uint256 scalar);
}

interface IFunctionGatewayErrors {
    error RequestNotFound(bytes32 requestId);
    error ContextMismatch(bytes32 contextHash, bytes context);
    error OutputMismatch(bytes32 outputHash, bytes context);
    error InputsRootMismatch(bytes32 inputsRoot, bytes32[] inputHashes);
    error OutputsRootMismatch(bytes32 outputsRoot, bytes32[] outputHashes);
    error VerificationKeysRootMismatch(
        bytes32 outputsRoot,
        bytes32[] outputHashes
    );
    error ProofNotFulfilled(bytes32 requestId);
    error ProofAlreadyFulfilled(bytes32 requestId);
    error InvalidProof(
        address verifier,
        bytes32 inputHash,
        bytes32 outputHash,
        bytes proof
    );
    error CallbackFailed(address callbackAddress, bytes4 callbackSelector);
    error CallbackAlreadyFulfilled(bytes32 requestId);
    error LengthMismatch(uint256 expected, uint256 actual);
    error InsufficientFeeAmount(uint256 expected, uint256 actual);
    error RefundFailed(address refundAccount, uint256 refundAmount);
}

interface IFunctionGateway is IFunctionGatewayEvents, IFunctionGatewayErrors {
    function requests(
        bytes32 requestId
    )
        external
        view
        returns (
            bytes32,
            bytes32,
            bytes32,
            bytes32,
            address,
            bytes4,
            bool,
            bool
        );

    function request(
        bytes32 functionId,
        bytes memory input,
        bytes4 callbackSelector,
        bytes memory context
    ) external payable returns (bytes32);

    function request(
        bytes32 functionId,
        bytes memory input,
        bytes4 callbackSelector,
        bytes memory context,
        uint256 gasLimit,
        address refundAccount
    ) external payable returns (bytes32);

    function fulfill(
        bytes32 requestId,
        bytes32 outputHash,
        bytes memory proof
    ) external;

    function fulfillBatch(
        bytes32[] memory requestIds,
        bytes memory aggregateProof,
        bytes32 inputsRoot,
        bytes32[] memory outputHashes,
        bytes32 outputsRoot,
        bytes32 verificationKeyRoot
    ) external;

    function callback(
        bytes32 requestId,
        bytes memory output,
        bytes memory context
    ) external;

    function calculateFeeAmount() external view returns (uint256);

    function calculateFeeAmount(
        uint256 gasLimit
    ) external view returns (uint256);
}
