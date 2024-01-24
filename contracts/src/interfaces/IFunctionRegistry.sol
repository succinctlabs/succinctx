// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0;

interface IFunctionRegistryEvents {
    event FunctionRegistered(
        bytes32 indexed functionId, address verifier, bytes32 salt, address owner
    );
    event FunctionVerifierUpdated(bytes32 indexed functionId, address verifier);
    event Deployed(
        bytes32 indexed bytecodeHash, bytes32 indexed salt, address indexed deployedAddress
    );
}

interface IFunctionRegistryErrors {
    error EmptyBytecode();
    error FailedDeploy();
    error VerifierCannotBeZero();
    error VerifierAlreadyUpdated(bytes32 functionId);
    error FunctionAlreadyRegistered(bytes32 functionId);
    error NotFunctionOwner(address owner, address actualOwner);
}

interface IFunctionRegistry is IFunctionRegistryEvents, IFunctionRegistryErrors {
    function verifiers(bytes32 functionId) external view returns (address verifier);
    function verifierOwners(bytes32 functionId) external view returns (address owner);
    function registerFunction(address owner, address verifier, bytes32 salt)
        external
        returns (bytes32 functionId);
    function deployAndRegisterFunction(address owner, bytes memory bytecode, bytes32 salt)
        external
        returns (bytes32 functionId, address verifier);
    function updateFunction(address verifier, bytes32 salt) external returns (bytes32 functionId);
    function deployAndUpdateFunction(bytes memory bytecode, bytes32 salt)
        external
        returns (bytes32 functionId, address verifier);
    function getFunctionId(address owner, bytes32 salt)
        external
        pure
        returns (bytes32 functionId);
}
