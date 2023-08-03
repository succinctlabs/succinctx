// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

import {IFunctionVerifier} from "src/interfaces/IFunctionVerifier.sol";

interface IFunctionRegistryErrors {
    error FunctionAlreadyRegistered(bytes32 proofId);
    error FunctionNotRegistered(bytes32 proofId);
    error NotFunctionOwner(address owner, address actualOwner);
}

interface IFunctionRegistry is IFunctionRegistryErrors {
    function verifiers(bytes32 functionId) external view returns (IFunctionVerifier);
    function verifierOwners(bytes32 functionId) external view returns (address);
    function registerFunction(bytes32 functionId, address verifier, address owner) external;
    function updateFunctionVerifier(bytes32 functionId, address verifier) external;
    function updateFunctionOwner(bytes32 functionId, address owner) external;
}
