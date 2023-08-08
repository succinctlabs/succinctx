// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.16;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @dev Identical to an ERC1967Proxy, with a more readable name.
contract Proxy is ERC1967Proxy {
    constructor(address _implementation, bytes memory _data) ERC1967Proxy(_implementation, _data) {}
}
