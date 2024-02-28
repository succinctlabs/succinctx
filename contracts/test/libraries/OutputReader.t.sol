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
        console.logBytes(output);
        uint256 offset = 0;
        uint128 value;
        (offset, value) = OutputReader.readUint128(output, 0);
        console.log("offset", offset);
        console.log("value", value);
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

    // function test_ReadUint64() public {
    //     bytes memory output = abi.encodePacked(uint128(1));
    //     console.logBytes(output);
    //     uint256 offset = 0;
    //     uint64 value;
    //     (offset, value) = OutputReader.readUint64(output, 0);
    //     console.log("offset", offset);
    //     console.log("value", value);
    //     assertEq(offset, 16);
    //     assertEq(value, 1);
    // }

    // function testFuzz_ReadUint64(uint128 v) public {
    //     bytes memory output = abi.encodePacked(v);
    //     uint256 offset = 0;
    //     uint64 value;
    //     (offset, value) = OutputReader.readUint64(output, 0);
    //     assertEq(offset, 16);
    //     assertEq(value, v);
    // }

    // function test_ReadUint64Multiple() public {
    //     bytes memory output = abi.encodePacked(uint128(1), uint128(2));
    //     uint256 offset = 0;
    //     uint64 value1;
    //     uint64 value2;
    //     (offset, value1) = OutputReader.readUint64(output, 0);
    //     assertEq(offset, 16);
    //     assertEq(value1, 1);
    //     (offset, value2) = OutputReader.readUint64(output, offset);
    //     assertEq(offset, 32);
    //     assertEq(value2, 2);
    // }

    // function test_ReadUint32() public {
    //     bytes memory output = abi.encodePacked(uint128(1));
    //     console.logBytes(output);
    //     uint256 offset = 0;
    //     uint32 value;
    //     (offset, value) = OutputReader.readUint32(output, 0);
    //     console.log("offset", offset);
    //     console.log("value", value);
    //     assertEq(offset, 16);
    //     assertEq(value, 1);
    // }

    // function testFuzz_ReadUint32(uint128 v) public {
    //     bytes memory output = abi.encodePacked(v);
    //     uint256 offset = 0;
    //     uint32 value;
    //     (offset, value) = OutputReader.readUint32(output, 0);
    //     assertEq(offset, 16);
    //     assertEq(value, v);
    // }

    // function test_ReadUint32Multiple() public {
    //     bytes memory output = abi.encodePacked(uint32(1), uint32(2));
    //     uint256 offset = 0;
    //     uint32 value1;
    //     uint32 value2;
    //     (offset, value1) = OutputReader.readUint32(output, 0);
    //     assertEq(offset, 16);
    //     assertEq(value1, 1);
    //     (offset, value2) = OutputReader.readUint32(output, offset);
    //     assertEq(offset, 32);
    //     assertEq(value2, 2);
    // }
}
