// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Vm.sol";
import "forge-std/console.sol";
import "forge-std/Test.sol";

import {OutputReader} from "../../src/libraries/OutputReader.sol";

contract OutputReaderTest is Test {
    function setUp() public {}

    function test_ReadUint256() public {
        bytes memory output = abi.encodePacked(uint256(1));
        uint256 offset = 0;
        uint256 value;
        (offset, value) = OutputReader.readUint256(output, 0);
        assertEq(offset, 32);
        assertEq(value, 1);
    }

    function testFuzz_ReadUint256(uint256 v) public {
        bytes memory output = abi.encodePacked(v);
        uint256 offset = 0;
        uint256 value;
        (offset, value) = OutputReader.readUint256(output, 0);
        assertEq(offset, 32);
        assertEq(value, v);
    }

    function test_ReadUint256Multiple() public {
        bytes memory output = abi.encodePacked(uint256(1), uint256(2));
        uint256 offset = 0;
        uint256 value1;
        uint256 value2;
        (offset, value1) = OutputReader.readUint256(output, 0);
        assertEq(offset, 32);
        assertEq(value1, 1);
        (offset, value2) = OutputReader.readUint256(output, offset);
        assertEq(offset, 64);
        assertEq(value2, 2);
    }

    function test_ReadUint128() public {
        bytes memory output = abi.encodePacked(uint128(1));
        uint256 offset = 0;
        uint128 value;
        (offset, value) = OutputReader.readUint128(output, 0);
        assertEq(offset, 16);
        assertEq(value, 1);
    }

    function testFuzz_ReadUint128(uint128 v) public {
        bytes memory output = abi.encodePacked(v);
        uint256 offset = 0;
        uint128 value;
        (offset, value) = OutputReader.readUint128(output, 0);
        assertEq(offset, 16);
        assertEq(value, v);
    }

    function test_ReadUint128Multiple() public {
        bytes memory output = abi.encodePacked(uint128(1), uint128(2));
        uint256 offset = 0;
        uint128 value1;
        uint128 value2;
        (offset, value1) = OutputReader.readUint128(output, 0);
        assertEq(offset, 16);
        assertEq(value1, 1);
        (offset, value2) = OutputReader.readUint128(output, offset);
        assertEq(offset, 32);
        assertEq(value2, 2);
    }
}
