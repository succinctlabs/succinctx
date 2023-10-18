// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/// @notice Script to inherit from to get access to helper functions
abstract contract BaseScript is Script {
    /// @notice Run the command with the `--broadcast` flag to send the transaction to the chain,
    ///         otherwise just simulate the transaction execution.
    modifier broadcaster() {
        vm.startBroadcast(msg.sender);
        _;
        vm.stopBroadcast();
    }

    string internal constant DEPLOYMENT_FILE = ".env.deployments";
    bytes32 internal constant PRIVATE_KEY = keccak256("PRIVATE_KEY");
    bytes32 internal constant LEDGER = keccak256("LEDGER");

    bytes32 internal walletType;
    bytes32 internal privateKey;
    uint256 internal mnemonicIndex;

    function setUp() public {
        // Inspired by:
        // https://github.com/ind-igo/forge-safe/blob/d322b7994d4bbd87b5d41c2143280bf433712f94/src/BatchScript.sol#L14
        walletType = keccak256(abi.encodePacked(vm.envString("WALLET_TYPE")));
        if (walletType == PRIVATE_KEY) {
            privateKey = vm.envBytes32("PRIVATE_KEY");
        } else if (walletType == LEDGER) {
            mnemonicIndex = vm.envUint("MNEMONIC_INDEX");
        } else {
            revert("Unsupported wallet type");
        }
    }

    function envUint256(string memory name) internal view returns (uint256) {
        uint256 value = vm.envUint(name);
        if (value == 0) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, value);
        return value;
    }

    function envBool(string memory name) internal view returns (bool) {
        bool value = vm.envBool(name);
        if (!value) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, value);
        return value;
    }

    function envBool(string memory name, bool default_) internal view returns (bool) {
        bool value = vm.envBool(name);
        if (!value) {
            return default_;
        }
        console.log("%s: %s", name, value);
        return value;
    }

    function envUint32(string memory name) internal view returns (uint32) {
        uint32 value = uint32(vm.envUint(name));
        if (value == 0) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, value);
        return value;
    }

    function envUint32s(string memory name, string memory delimiter)
        internal
        returns (uint32[] memory)
    {
        uint256[] memory values = new uint256[](0);
        values = vm.envOr(name, delimiter, values);
        if (values.length == 0) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s:", name);
        for (uint256 i = 0; i < values.length; i++) {
            console.log("  %s", values[i]);
        }
        uint32[] memory converted = new uint32[](values.length);
        for (uint256 i = 0; i < values.length; i++) {
            converted[i] = uint32(values[i]);
        }
        return converted;
    }

    function envUint64(string memory name) internal view returns (uint64) {
        uint64 value = uint64(vm.envUint(name));
        if (value == 0) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, value);
        return value;
    }

    function envBytes32(string memory name) internal view returns (bytes32) {
        bytes32 value = vm.envBytes32(name);
        if (value == bytes32(0)) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, Strings.toHexString(uint256(value)));
        return value;
    }

    function envAddress(string memory name) internal view returns (address) {
        address addr = vm.envAddress(name);
        if (addr == address(0)) {
            console.log("%s is not set", name);
            revert();
        }
        console.log("%s: %s", name, addr);
        return addr;
    }

    function envAddress(string memory name, uint256 chainId) internal returns (address) {
        string memory envName = string.concat(name, "_", Strings.toString(chainId));
        address addr = vm.envOr(envName, address(0));
        if (addr == address(0)) {
            //try without chainId
            addr = vm.envOr(name, address(0));
            if (addr == address(0)) {
                console.log("%s/%s is not set", envName, name);
                revert();
            }
        }
        console.log("%s: %s", envName, addr);
        return addr;
    }

    function envAddresses(string memory name, uint256 chainId, string memory delimiter)
        internal
        returns (address[] memory)
    {
        string memory envName = string.concat(name, "_", Strings.toString(chainId));
        address[] memory addresses = new address[](0);
        addresses = vm.envOr(envName, delimiter, addresses);
        if (addresses.length == 0 || addresses[0] == address(0)) {
            //try without chainId
            addresses = vm.envOr(name, delimiter, addresses);
            if (addresses.length == 0 || addresses[0] == address(0)) {
                console.log("%s/%s is not set", envName, name);
                revert();
            }
        }
        for (uint256 i = 0; i < addresses.length; i++) {
            console.log("%s: %s", envName, addresses[i]);
        }
        return addresses;
    }

    function writeEnvAddress(string memory file, string memory name, address value) internal {
        string memory addrVar = string.concat(name, "_", Strings.toString(block.chainid));
        vm.setEnv(addrVar, Strings.toHexString(value));
        vm.writeLine(file, string.concat(string.concat(addrVar, "="), Strings.toHexString(value)));
        console.log(string.concat(string.concat(addrVar, "="), Strings.toHexString(value)));
    }

    function writeEnvAddresses(
        string memory file,
        string memory name,
        address[] memory values,
        string memory delimiter
    ) internal {
        string memory addrVar = string.concat(name, "_", Strings.toString(block.chainid));
        string memory line = string.concat(addrVar, "=");
        string memory addrs;
        for (uint256 i = 0; i < values.length; i++) {
            addrs = string.concat(addrs, Strings.toHexString(values[i]));
            if (i < values.length - 1) {
                addrs = string.concat(addrs, delimiter);
            }
        }
        line = string.concat(line, addrs);
        vm.setEnv(addrVar, addrs);
        vm.writeLine(file, line);
        console.log(line);
    }

    /// @notice Use 'cast wallet sign' to sign a message.
    /// @dev Needed because internal vm.sign has needs access to the private key directly,
    ///      which is unavailable for hardward wallets.
    ///
    ///      Keep in mind cast wallet sign uses EIP-191 eth_sign: https://eips.ethereum.org/EIPS/eip-191
    ///      with the message prefixed with "\x19Ethereum Signed Message:\n" + message.length. To work
    ///      around this, we add 4 to the last byte of the signature.
    function sign(string memory _message) internal returns (bytes memory signature) {
        string memory commandStart = "cast wallet sign ";
        string memory flags;
        if (walletType == PRIVATE_KEY) {
            flags = string.concat("--private-key ", vm.toString(privateKey), " ");
        } else if (walletType == LEDGER) {
            flags = string.concat("--ledger --mnemonic-index ", vm.toString(mnemonicIndex), " ");
        } else {
            revert("Unsupported wallet type");
        }

        string[] memory inputs = new string[](3);
        inputs[0] = "bash";
        inputs[1] = "-c";
        inputs[2] = string.concat(commandStart, flags, _message);
        signature = add4(vm.ffi(inputs));
    }

    function getSigner() internal returns (address signer) {
        string memory commandStart = "cast wallet address ";
        string memory flags;
        if (walletType == PRIVATE_KEY) {
            flags = string.concat("--private-key ", vm.toString(privateKey), " ");
        } else if (walletType == LEDGER) {
            flags = string.concat("--ledger --mnemonic-index ", vm.toString(mnemonicIndex), " ");
        } else {
            revert("Unsupported wallet type");
        }

        string[] memory inputs = new string[](3);
        inputs[0] = "bash";
        inputs[1] = "-c";
        inputs[2] = string.concat(commandStart, flags);
        signer = address(uint160(bytes20(vm.ffi(inputs))));
    }

    function add4(bytes memory _b) private pure returns (bytes memory) {
        require(_b.length > 0, "byte array must not be empty");

        uint256 lastByte = uint256(uint8(_b[_b.length - 1]));
        uint256 incrementedLastByte = lastByte + 4;

        if (incrementedLastByte > 255) {
            incrementedLastByte = 255;
        }

        _b[_b.length - 1] = bytes1(uint8(incrementedLastByte));

        return _b;
    }

    function buildSignaturesFromArray(bytes[] memory _signatures)
        internal
        pure
        returns (bytes memory)
    {
        bytes memory signatures;
        for (uint256 i = 0; i < _signatures.length; i++) {
            signatures = bytes.concat(signatures, bytes(_signatures[i]));
        }
        return signatures;
    }
}
