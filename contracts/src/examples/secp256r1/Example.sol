// SPDX-License-Identifier: MIT

pragma solidity ^0.8.16;

import {Secp256r1} from "./Secp256r1.sol";

contract Example {
    function a() external {
        address gateway = address(0);
        bytes32 functionId = bytes32(0);
        uint256 r = 0;
        uint256 s = 0;
        bytes4 callbackSelector = Example.b.selector;
        bytes memory context = abi.encode(r, s);
        Secp256r1.request(gateway, functionId, r, s, callbackSelector, context);
    }

    function b(bytes memory _output, bytes memory _context) external pure returns (uint256) {
        bool verified = Secp256r1.decode(_output);
        (uint256 r, uint256 s) = abi.decode(_context, (uint256, uint256));
        if (verified) {
            return r + s;
        }
        return 0;
    }
}
