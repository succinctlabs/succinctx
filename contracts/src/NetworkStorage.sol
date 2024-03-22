// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {WhitelistStatus} from "./interfaces/ISuccinctGateway.sol";

abstract contract NetworkStorage {
    address public feeVault;

    uint32 public nonce;

    bool internal executingCallback;

    address internal verifiedVerifier;

    bytes32 internal verifiedInputHash;

    bytes internal verifiedOutput;

    mapping(uint32 => bytes32) public requests;

    mapping(address => address) public verifierOwners;

    mapping(address => WhitelistStatus) public whitelistStatus;

    mapping(address => mapping(address => bool)) public allowedProvers;
}
