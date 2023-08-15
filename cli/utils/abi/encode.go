package abi

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
)

// Emulates Solidity's abi.encodePacked() function. Must give the type signature and the type values.
// The type signature must be in the form of "(<type1>,<type2>,...)".
// The type values must be in the form of ["<value1>", "<value2>", ...].
func EncodePacked(signature string, values []string) ([]byte, error) {
	if signature == "()" {
		if len(values) != 0 {
			return nil, fmt.Errorf("mismatch in number of types and values")
		}
		return []byte{}, nil
	}

	types := strings.Split(signature[1:len(signature)-1], ",")
	if len(types) != len(values) {
		return nil, fmt.Errorf("mismatch in number of types and values")
	}

	var buffer bytes.Buffer
	for i, t := range types {
		switch t {
		case "bytes32":
			bytesVal, err := hexutil.Decode(values[i])
			if err != nil || len(bytesVal) != 32 {
				return nil, fmt.Errorf("invalid bytes32 value: %s", values[i])
			}
			buffer.Write(bytesVal)
		case "uint256":
			val := big.NewInt(0)
			val.SetString(values[i], 10)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(val.Bytes(), 32))
		case "uint160":
			val := big.NewInt(0)
			val.SetString(values[i], 10)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(val.Bytes(), 20))
		case "uint128":
			val := big.NewInt(0)
			val.SetString(values[i], 10)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(val.Bytes(), 16))
		case "uint64":
			val := uint64(0)
			fmt.Sscanf(values[i], "%d", &val)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(common.BigToHash(big.NewInt(int64(val))).Bytes(), 8))
		case "uint32":
			val := uint32(0)
			fmt.Sscanf(values[i], "%d", &val)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(common.BigToHash(big.NewInt(int64(val))).Bytes(), 4))
		case "uint16":
			val := uint16(0)
			fmt.Sscanf(values[i], "%d", &val)
			binary.Write(&buffer, binary.BigEndian, common.LeftPadBytes(common.BigToHash(big.NewInt(int64(val))).Bytes(), 2))
		case "uint8":
			val := uint8(0)
			fmt.Sscanf(values[i], "%d", &val)
			buffer.WriteByte(val)
		case "address":
			buffer.Write(common.HexToAddress(values[i]).Bytes())
		case "bool":
			if values[i] == "true" {
				buffer.WriteByte(0x01)
			} else {
				buffer.WriteByte(0x00)
			}
		case "bytes":
			buffer.Write([]byte(values[i]))
		case "string":
			buffer.Write([]byte(values[i]))
		default:
			return nil, fmt.Errorf("unsupported type: %s", t)
		}
	}

	return buffer.Bytes(), nil
}
