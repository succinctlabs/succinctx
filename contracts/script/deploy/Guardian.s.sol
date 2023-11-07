// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/console.sol";
import {BaseScript} from "../misc/Base.s.sol";
import {Safe} from "@safe/Safe.sol";
import {SafeProxyFactory} from "@safe/proxies/SafeProxyFactory.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

// "Guardian" refers to a Gnosis Safe proxy.
contract DeployGuardian is BaseScript {
    function run() external broadcaster {
        console.log(
            "Deploying Guardian (Safe) contract on chain %s", Strings.toString(block.chainid)
        );

        // Check inputs

        bytes32 CREATE2_SALT = envBytes32("CREATE2_SALT");
        address[] memory GUARDIAN_OWNERS = envAddresses("GUARDIAN_OWNERS", block.chainid, ",");

        // For testing we create new singleton + factory. Generally it is better to use the existing
        // deployment if it exists for the chain. Check:
        // https://github.com/safe-global/safe-deployments/tree/73fa0411e26ab6c3dd80776943d9f1ba7328bb72/src/assets
        Safe safeSingleton = new Safe();
        SafeProxyFactory safeFactory = new SafeProxyFactory();

        Safe safe = createSafeProxy(safeSingleton, safeFactory, CREATE2_SALT, GUARDIAN_OWNERS);

        // Write address
        writeEnvAddress(DEPLOYMENT_FILE, "GUARDIAN", address(safe));
        writeEnvAddress(DEPLOYMENT_FILE, "GUARDIAN_IMPL", address(safeSingleton));
    }

    function createSafeProxy(
        Safe _safeSingleton,
        SafeProxyFactory _safeFactory,
        bytes32 _salt,
        address[] memory _owners
    ) public returns (Safe) {
        bytes memory initializer = abi.encodeWithSignature(
            "setup(address[],uint256,address,bytes,address,address,uint256,address)",
            _owners,
            _owners.length,
            payable(address(0)),
            0x0,
            address(0),
            address(0),
            0,
            address(0)
        );

        return Safe(
            payable(
                _safeFactory.createProxyWithNonce(
                    address(_safeSingleton), initializer, uint256(_salt)
                )
            )
        );
    }
}
