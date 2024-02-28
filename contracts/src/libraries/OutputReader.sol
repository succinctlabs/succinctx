pragma solidity ^0.8.16;

// OutputReader should be used for reading encodePacked output uint256, uint128, uint64, uint32.
library OutputReader {
    function readUint256(bytes memory _output, uint256 _offset)
        internal
        pure
        returns (uint256, uint256)
    {
        uint256 value;
        assembly {
            value := mload(add(add(_output, 0x20), _offset))
        }
        return (_offset + 32, value);
    }

    function readUint128(bytes memory _output, uint256 _offset)
        internal
        pure
        returns (uint256, uint128)
    {
        uint128 value;
        assembly {
            // Reading into u128 only parses the lowest 16 bytes of slot, this is where
            // encodePacked writes the uint128.
            value := mload(add(add(_output, 0x10), _offset))
        }
        return (_offset + 16, value);
    }

    function readUint64(bytes memory _output, uint256 _offset)
        internal
        pure
        returns (uint256, uint64)
    {
        uint64 value;
        assembly {
            // Reading into u64 only parses the lowest 8 bytes of slot, this is where
            // encodePacked writes the uint64.
            value := mload(add(add(_output, 0x08), _offset))
        }
        return (_offset + 8, value);
    }

    function readUint32(bytes memory _output, uint256 _offset)
        internal
        pure
        returns (uint256, uint32)
    {
        uint32 value;
        assembly {
            // Reading into u32 only parses the lowest 4 bytes of slot, this is where
            // encodePacked writes the uint32.
            value := mload(add(add(_output, 0x04), _offset))
        }
        return (_offset + 4, value);
    }
}
