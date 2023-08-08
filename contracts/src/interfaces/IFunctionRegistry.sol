// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

interface IFunctionRegistryEvents {
    event FunctionRegistered(bytes32 indexed functionId, address verifier, string name, address owner);
    event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier);
    event FunctionOwnerUpdated(bytes32 indexed functionId, address owner);
    event Deployed(bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress);
}

interface IFunctionRegistryErrors {
    error EmptyBytecode();
    error FailedDeploy();
    error FunctionAlreadyHasVerifier(bytes32 functionId);
    error NotFunctionOwner(address owner, address actualOwner);
}

interface IFunctionRegistry is IFunctionRegistryEvents, IFunctionRegistryErrors {
    function verifiers(bytes32 functionId) external view returns (address verifier);
    function verifierOwners(bytes32 functionId) external view returns (address owner);
    function registerFunction(bytes memory bytecode, string memory name) external returns (address verifier);
    function updateFunction(bytes memory bytecode, string memory name) external returns (address verifier);
    function getFunctionId(address owner, string memory name) external pure returns (bytes32 functionId);
}
