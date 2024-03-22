// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

interface INetworkVerifier {
    function verify(
        bytes32 programHash,
        bytes32 inputHash,
        bytes32 outputHash,
        bytes calldata proof
    ) external returns (bool);

    function verificationKeyHash() external view returns (bytes32);
}
