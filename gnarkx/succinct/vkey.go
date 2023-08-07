package succinct

import (
	"bytes"
	"io"
	"strings"

	"github.com/consensys/gnark/backend/groth16"
)

type SuccinctVerifyingKey struct {
	groth16.VerifyingKey
}

func (svk *SuccinctVerifyingKey) ExportIFunctionVerifierSolidity(w io.Writer) error {
	// Create a new buffer and export the VerifyingKey into it as a Solidity contract and
	// convert the buffer content to a string for further manipulation.
	buf := new(bytes.Buffer)
	err := svk.VerifyingKey.ExportSolidity(buf)
	if err != nil {
		return err
	}
	content := buf.String()

	// Custom replacements to make compatible with IFunctionVerifier.
	content = strings.ReplaceAll(content, "uint256[2] calldata input", "uint256[2] memory input")
	content = strings.ReplaceAll(content, "pragma solidity ^0.8.0;", "pragma solidity ^0.8.16;")
	// write the new content to the writer
	_, err = w.Write([]byte(content))
	if err != nil {
		return err
	}

	// Generate the IFunctionVerifier interface and FunctionVerifier contract.
	solidityIFunctionVerifier := `
interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, Verifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) =
            abi.decode(_proof, (uint256[2], uint256[2][2], uint256[2]));

		uint256[2] memory input = [uint256(_inputHash), uint256(_outputHash)];

		return verifyProof(a, b, c, input);
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256(abi.encode(verifyingKey()));
    }
}
`
	// write the IFunctionVerifier and FunctionVerifier to the writer
	_, err = w.Write([]byte(solidityIFunctionVerifier))
	return err
}
