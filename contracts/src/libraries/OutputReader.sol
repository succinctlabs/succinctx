pragma solidity ^0.8.16;

library OutputReader {
    function readUint256(bytes memory _output, uint256 _offset) internal pure returns (uint256, uint256) {
        uint256 value;
        assembly {
            value := mload(add(add(_output, 0x20), _offset))
        }
        return (_offset + 32, value);
    }

    function readUint64(bytes memory _output, uint256 _offset) internal pure returns (uint256, uint64) {
        uint64 value;
        assembly {
            value := mload(add(add(_output, 0x08), _offset))
        }
        return (_offset + 8, value);
    }
}
