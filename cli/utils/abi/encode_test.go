package abi

import (
	"bytes"
	"testing"

	"github.com/ethereum/go-ethereum/common/hexutil"
)

func TestEncodePacked(t *testing.T) {
	tests := []struct {
		signature string
		values    []string
		expected  hexutil.Bytes
		err       bool
	}{
		{
			"(uint256,address,uint8,bool,string)",
			[]string{"23123", "0xDEd0000E32f8F40414d3ab3a830f735a3553E18e", "4", "true", "hello"},
			hexutil.MustDecode("0x0000000000000000000000000000000000000000000000000000000000005a53ded0000e32f8f40414d3ab3a830f735a3553e18e040168656c6c6f"),
			false,
		},
		{
			"(bytes32)",
			[]string{"0x123456789012345678901234567890123456789012345678901234567890abcd"},
			hexutil.MustDecode("0x123456789012345678901234567890123456789012345678901234567890abcd"),
			false,
		},
		{
			"(bytes32,uint256)",
			[]string{"0x123456789012345678901234567890123456789012345678901234567890abcd", "42"},
			hexutil.MustDecode("0x123456789012345678901234567890123456789012345678901234567890abcd000000000000000000000000000000000000000000000000000000000000002a"), // 32 bytes for bytes32, 32 bytes for uint256
			false,
		},
		{
			"(bytes)",
			[]string{"hello"},
			[]byte("hello"),
			false,
		},
		{
			"(string)",
			[]string{"world"},
			[]byte("world"),
			false,
		},
		{
			"(uint256,bool)",
			[]string{"0", "false"},
			hexutil.MustDecode("0x000000000000000000000000000000000000000000000000000000000000000000"), // 32 bytes of zero, plus 1 byte of zero
			false,
		},
		{
			"(string,uint8)",
			[]string{"test", "255"},
			hexutil.MustDecode("0x74657374ff"), // "test" as ASCII, followed by 255 as a single byte
			false,
		},
		{
			"(uint256,address)",
			[]string{"1"},
			nil,
			true, // Mismatch in number of types and values
		},
		{
			"(unsupportedType)",
			[]string{"value"},
			nil,
			true, // Unsupported type
		},
		{
			"()",
			[]string{},               // Empty values slice
			hexutil.MustDecode("0x"), // Empty byte slice
			false,
		},
		{
			"(uint256)",
			[]string{"115792089237316195423570985008687907853269984665640564039457584007913129639935"}, // 2^256 - 1
			hexutil.MustDecode("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),   // 32 bytes of 0xff
			false,
		},
	}

	for _, test := range tests {
		result, err := EncodePacked(test.signature, test.values)
		if (err != nil) != test.err {
			t.Errorf("Unexpected error for signature %s: %v", test.signature, err)
			continue
		}
		if !bytes.Equal(result, test.expected) {
			t.Errorf("Unexpected result for signature %s:\n- got (hex): %x\n- got (bytes): %v\n- want (hex): %s\n- want (bytes): %v", test.signature, result, result, hexutil.Encode(test.expected), test.expected)
		}
	}
}
