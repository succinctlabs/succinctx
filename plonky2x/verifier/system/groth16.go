package system

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math/big"
	"os"
	"strings"
	"time"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	groth16_bn254 "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/backend/witness"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/pkg/errors"
	"github.com/rs/zerolog"

	gnark_verifier_types "github.com/succinctlabs/gnark-plonky2-verifier/types"
	"github.com/succinctlabs/gnark-plonky2-verifier/variables"
)

type Groth16System struct {
	logger      zerolog.Logger
	circuitPath string
	dataPath    string
}

func NewGroth16System(logger zerolog.Logger, circuitPath string, dataPath string) *Groth16System {
	return &Groth16System{
		logger:      logger,
		circuitPath: circuitPath,
		dataPath:    dataPath,
	}
}

func (s *Groth16System) Compile() error {
	s.logger.Info().Msg("starting compiling verifier circuit")

	r1cs, pk, vk, err := s.CompileVerifierCircuit()
	if err != nil {
		return errors.Wrap(err, "compile verifier circuit")
	}

	err = s.SaveVerifierCircuit(r1cs, pk, vk)
	if err != nil {
		return errors.Wrap(err, "save verifier circuit")
	}

	s.logger.Info().Msg("successfully compiled verifier circuit")

	return nil
}

func (s *Groth16System) Prove() error {
	s.logger.Info().Msg("starting prove -- loading verifier circuit and proving key")

	r1cs, err := s.LoadCircuit()
	if err != nil {
		return errors.Wrap(err, "load the verifier circuit")
	}
	pk, err := s.LoadProvingKey()
	if err != nil {
		return errors.Wrap(err, "load the proving key")
	}

	// If the circuitPath is "" and not provided as part of the CLI flags, then we wait
	// for user input.
	if s.circuitPath == "" {
		s.logger.Info().Msg("no circuitPath flag found, so user must input circuitPath via stdin")
		reader := bufio.NewReader(os.Stdin)
		str, err := reader.ReadString('\n')
		if err != nil {
			return errors.Wrap(err, "read circuitPath from stdin")
		}
		trimmed := strings.TrimSuffix(str, "\n")
		s.circuitPath = trimmed
	}

	s.logger.Info().Msgf("generating proof with circuit path %v", s.circuitPath)
	_, _, err = s.ProveCircuit(r1cs, pk)
	if err != nil {
		return errors.Wrap(err, "create proof")
	}

	s.logger.Info().Msg("successfully created proof")

	return nil
}

func (s *Groth16System) Verify() error {
	s.logger.Info().Msg("starting verify -- loading verifier key, public witness, and proof")

	vk, err := s.LoadVerifierKey()
	if err != nil {
		return errors.Wrap(err, "load verifier key")
	}

	proof, err := s.LoadProof()
	if err != nil {
		return errors.Wrap(err, "load proof")
	}

	publicWitness, err := s.LoadPublicWitness()
	if err != nil {
		return errors.Wrap(err, "load public witness")
	}

	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		return errors.Wrap(err, "verify proof")
	}

	s.logger.Info().Msg("successfully verified proof")

	return nil
}

func (s *Groth16System) Export() error {
	s.logger.Info().Msg("starting export -- loading verifier key and exporting Verifier solidity")

	vk, err := s.LoadVerifierKey()
	if err != nil {
		return errors.Wrap(err, "load verifier key")
	}

	err = s.ExportVerifierJSON(vk)
	if err != nil {
		return errors.Wrap(err, "export Verifier JSON")
	}

	err = s.ExportVerifierSolidity(vk)
	if err != nil {
		return errors.Wrap(err, "export Verifier solidity")
	}

	s.logger.Info().Msg("successfully exported Verifier solidity")

	return nil
}

func (s *Groth16System) CompileVerifierCircuit() (constraint.ConstraintSystem, groth16.ProvingKey, groth16.VerifyingKey, error) {
	verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
		gnark_verifier_types.ReadVerifierOnlyCircuitData(s.circuitPath + "/verifier_only_circuit_data.json"),
	)
	proofWithPis := variables.DeserializeProofWithPublicInputs(
		gnark_verifier_types.ReadProofWithPublicInputs(s.circuitPath + "/proof_with_public_inputs.json"),
	)
	commonCircuitData := gnark_verifier_types.ReadCommonCircuitData(s.circuitPath + "/common_circuit_data.json")

	circuit := VerifierCircuit{
		ProofWithPis:      proofWithPis,
		VerifierData:      verifierOnlyCircuitData,
		VerifierDigest:    new(frontend.Variable),
		InputHash:         new(frontend.Variable),
		OutputHash:        new(frontend.Variable),
		CommonCircuitData: commonCircuitData,
	}
	r1cs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		return nil, nil, nil, errors.Wrap(err, "compile verifier circuit")
	}

	s.logger.Info().Msg("Running circuit setup")
	start := time.Now()
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		return nil, nil, nil, err
	}
	elapsed := time.Since(start)
	s.logger.Info().Msg("Successfully ran circuit setup in " + elapsed.String())

	return r1cs, pk, vk, nil
}

func (s *Groth16System) SaveVerifierCircuit(r1cs constraint.ConstraintSystem, pk groth16.ProvingKey, vk groth16.VerifyingKey) error {
	os.MkdirAll(s.dataPath, 0755)

	r1csFile, err := os.Create(s.dataPath + "/r1cs.bin")
	if err != nil {
		return errors.Wrap(err, "create r1cs file")
	}
	r1cs.WriteTo(r1csFile)
	r1csFile.Close()
	s.logger.Info().Msg("Successfully saved circuit constraints to r1cs.bin")

	s.logger.Info().Msg("Saving proving key to pk.bin")
	pkFile, err := os.Create(s.dataPath + "/pk.bin")
	if err != nil {
		return errors.Wrap(err, "create pk file")
	}
	pk.WriteRawTo(pkFile)
	pkFile.Close()
	s.logger.Info().Msg("Successfully saved proving key to pk.bin")

	vkFile, err := os.Create(s.dataPath + "/vk.bin")
	if err != nil {
		return errors.Wrap(err, "create vk file")
	}
	vk.WriteRawTo(vkFile)
	vkFile.Close()
	s.logger.Info().Msg("Successfully saved verifying key to vk.bin")

	return nil
}

func (s *Groth16System) ProveCircuit(r1cs constraint.ConstraintSystem, pk groth16.ProvingKey) (groth16.Proof, witness.Witness, error) {
	s.logger.Info().Msg("Loading verifier only circuit data and proof with public inputs in path " + s.circuitPath)
	verifierOnlyCircuitData := variables.DeserializeVerifierOnlyCircuitData(
		gnark_verifier_types.ReadVerifierOnlyCircuitData(s.circuitPath + "/verifier_only_circuit_data.json"),
	)
	proofWithPis := gnark_verifier_types.ReadProofWithPublicInputs(s.circuitPath + "/proof_with_public_inputs.json")
	proofWithPisVariable := variables.DeserializeProofWithPublicInputs(proofWithPis)

	inputHash, outputHash := GetInputHashOutputHash(proofWithPis)

	// Circuit assignment
	assignment := &VerifierCircuit{
		ProofWithPis:   proofWithPisVariable,
		VerifierData:   verifierOnlyCircuitData,
		VerifierDigest: verifierOnlyCircuitData.CircuitDigest,
		InputHash:      frontend.Variable(inputHash),
		OutputHash:     frontend.Variable(outputHash),
	}

	s.logger.Info().Msg("Generating witness")
	start := time.Now()
	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		return nil, nil, errors.Wrap(err, "generate witness")
	}
	elapsed := time.Since(start)
	s.logger.Info().Msg("Successfully generated witness in " + elapsed.String())

	s.logger.Info().Msg("Creating proof")
	start = time.Now()
	proof, err := groth16.Prove(r1cs, pk, witness)
	if err != nil {
		return nil, nil, errors.Wrap(err, "create proof")
	}
	elapsed = time.Since(start)
	s.logger.Info().Msg("Successfully created proof in " + elapsed.String())

	_proof := proof.(*groth16_bn254.Proof)
	s.logger.Info().Msg("Saving proof to proof.json")
	jsonProof, err := json.Marshal(ProofResult{
		Output: []byte{},
		Proof:  _proof.Ar.Marshal(),
	})
	if err != nil {
		return nil, nil, errors.Wrap(err, "marshal proof")
	}
	proofFile, err := os.Create("proof.json")
	if err != nil {
		return nil, nil, errors.Wrap(err, "create proof file")
	}
	defer proofFile.Close()
	if _, err = proofFile.Write(jsonProof); err != nil {
		return nil, nil, errors.Wrap(err, "write proof file")
	}
	s.logger.Info().Msg("Successfully saved proof")

	const fpSize = 4 * 8

	proofBytes := _proof.Ar.Marshal()

	// Ensure proofBytes contains enough data for the expected operation
	expectedLength := fpSize * 8
	if len(proofBytes) < expectedLength {
		return nil, nil, fmt.Errorf("proofBytes length is %d, expected at least %d", len(proofBytes), expectedLength)
	}

	proofs := make([]string, 8)
	for i := 0; i < 8; i++ {
		start := i * fpSize
		end := (i + 1) * fpSize
		// Additional check to prevent slice bounds out of range panic
		if end > len(proofBytes) {
			return nil, nil, fmt.Errorf("attempt to slice beyond proofBytes length at segment %d", i)
		}
		proofs[i] = "0x" + hex.EncodeToString(proofBytes[start:end])
	}

	publicWitness, _ := witness.Public()
	publicWitnessBytes, _ := publicWitness.MarshalBinary()
	publicWitnessBytes = publicWitnessBytes[12:] // We cut off the first 12 bytes because they encode length information

	inputs := make([]string, 3)
	// Print out the public witness bytes
	for i := 0; i < 3; i++ {
		inputs[i] = "0x" + hex.EncodeToString(publicWitnessBytes[i*fpSize:(i+1)*fpSize])
	}

	// Write proof with all the public inputs and save to disk.
	jsonProofWithWitness, err := json.Marshal(struct {
		Inputs         []string      `json:"inputs"`
		Proofs         []string      `json:"proofs"`
		InputHash      hexutil.Bytes `json:"input_hash"`
		OutputHash     hexutil.Bytes `json:"output_hash"`
		VerifierDigest hexutil.Bytes `json:"verifier_digest"`
		Proof          hexutil.Bytes `json:"proof"`
	}{
		Inputs:         inputs,
		Proofs:         proofs,
		InputHash:      inputHash.Bytes(),
		OutputHash:     outputHash.Bytes(),
		VerifierDigest: (verifierOnlyCircuitData.CircuitDigest).(*big.Int).Bytes(),
		Proof:          _proof.Ar.Marshal(),
	})
	if err != nil {
		return nil, nil, errors.Wrap(err, "marshal proof with witness")
	}
	proofFile, err = os.Create("proof_with_witness.json")
	if err != nil {
		return nil, nil, errors.Wrap(err, "create proof_with_witness file")
	}
	defer proofFile.Close()
	if _, err = proofFile.Write(jsonProofWithWitness); err != nil {
		return nil, nil, errors.Wrap(err, "write proof_with_witness file")
	}
	s.logger.Info().Msg("Successfully saved proof_with_witness to proof_with_witness.json")

	publicWitness, err = witness.Public()
	if err != nil {
		return nil, nil, errors.Wrap(err, "get public witness")
	}

	s.logger.Info().Msg("Saving public witness to public_witness.bin")
	witnessFile, err := os.Create("public_witness.bin")
	if err != nil {
		return nil, nil, errors.Wrap(err, "create public witness file")
	}
	defer witnessFile.Close()
	if _, err = publicWitness.WriteTo(witnessFile); err != nil {
		return nil, nil, errors.Wrap(err, "write public witness file")
	}
	s.logger.Info().Msg("Successfully saved public witness")

	return proof, publicWitness, nil
}

func (s *Groth16System) ExportVerifierSolidity(vk groth16.VerifyingKey) error {
	// Create a new buffer and export the VerifyingKey into it as a Solidity contract and
	// convert the buffer content to a string for further manipulation.
	buf := new(bytes.Buffer)
	err := vk.ExportSolidity(buf)
	if err != nil {
		return errors.Wrap(err, "export verifying key to solidity")
	}
	content := buf.String()

	contractFile, err := os.Create(s.dataPath + "/Verifier.sol")
	if err != nil {
		return errors.Wrap(err, "create Verifier.sol file")
	}
	defer contractFile.Close()

	w := bufio.NewWriter(contractFile)
	// write the new content to the writer
	if _, err = w.Write([]byte(content)); err != nil {
		return errors.Wrap(err, "write to Verifier.sol")
	}

	if err = w.Flush(); err != nil {
		return errors.Wrap(err, "flush writer for Verifier.sol")
	}

	return nil
}

func (s *Groth16System) ExportVerifierJSON(vk groth16.VerifyingKey) error {
	vkFile, err := os.Create(s.dataPath + "/vk.json")
	if err != nil {
		return errors.Wrap(err, "create vk.json file")
	}
	defer vkFile.Close()

	return nil
}

func (s *Groth16System) LoadCircuit() (constraint.ConstraintSystem, error) {
	r1cs := groth16.NewCS(ecc.BN254)
	f, err := os.Open(s.dataPath + "/r1cs.bin")
	if err != nil {
		return nil, errors.Wrap(err, "open r1cs file")
	}
	r1csReader := bufio.NewReader(f)
	_, err = r1cs.ReadFrom(r1csReader)
	if err != nil {
		return nil, errors.Wrap(err, "read r1cs file")
	}
	f.Close()

	return r1cs, nil
}

func (s *Groth16System) LoadProvingKey() (pk groth16.ProvingKey, err error) {
	pk = groth16.NewProvingKey(ecc.BN254)
	f, err := os.Open(s.dataPath + "/pk.bin")
	if err != nil {
		return pk, errors.Wrap(err, "open pk file")
	}
	_, err = pk.ReadFrom(f)
	if err != nil {
		return pk, errors.Wrap(err, "read pk file")
	}
	f.Close()

	return pk, nil
}

func (s *Groth16System) LoadVerifierKey() (vk groth16.VerifyingKey, err error) {
	vk = groth16.NewVerifyingKey(ecc.BN254)
	f, err := os.Open(s.dataPath + "/vk.bin")
	if err != nil {
		return nil, errors.Wrap(err, "open vk file")
	}
	_, err = vk.ReadFrom(f)
	if err != nil {
		return nil, errors.Wrap(err, "read vk file")
	}
	f.Close()

	return vk, nil
}

func (s *Groth16System) LoadProof() (proof groth16.Proof, err error) {
	proof = groth16.NewProof(ecc.BN254)
	f, err := os.Open(s.dataPath + "/proof.json")
	if err != nil {
		return proof, errors.Wrap(err, "open proof file")
	}
	jsonProof, err := io.ReadAll(f)
	if err != nil {
		return proof, errors.Wrap(err, "read proof file")
	}
	err = json.Unmarshal(jsonProof, proof)
	if err != nil {
		return proof, errors.Wrap(err, "read proof file")
	}
	f.Close()

	return proof, nil
}

func (s *Groth16System) LoadPublicWitness() (witness.Witness, error) {
	publicWitness, err := witness.New(ecc.BN254.ScalarField())
	if err != nil {
		return publicWitness, errors.Wrap(err, "create public witness")
	}
	f, err := os.Open(s.dataPath + "/public_witness.bin")
	if err != nil {
		return publicWitness, errors.Wrap(err, "open public witness file")
	}
	_, err = publicWitness.ReadFrom(f)
	if err != nil {
		return publicWitness, errors.Wrap(err, "read public witness file")
	}
	f.Close()

	return publicWitness, nil
}
