package system

import (
	"bufio"
	"bytes"
	"encoding/json"
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

	r1cs, err := s.LoadCircuit()
	if err != nil {
		return errors.Wrap(err, "load the verifier circuit")
	}
	pk, err := s.LoadProvingKey()
	if err != nil {
		return errors.Wrap(err, "load the proving key")
	}

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

	// Write proof with all the public inputs and save to disk.
	jsonProofWithWitness, err := json.Marshal(struct {
		InputHash      hexutil.Bytes `json:"input_hash"`
		OutputHash     hexutil.Bytes `json:"output_hash"`
		VerifierDigest hexutil.Bytes `json:"verifier_digest"`
		Proof          hexutil.Bytes `json:"proof"`
	}{
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

	publicWitness, err := witness.Public()
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
	vkw := VerifyingKeyWrapper{vk.(*groth16_bn254.VerifyingKey)}
	vkFile, err := os.Create(s.dataPath + "/vk.json")
	if err != nil {
		return errors.Wrap(err, "create vk.json file")
	}
	defer vkFile.Close()

	err = vkw.WriteJSONTo(vkFile)
	if err != nil {
		return errors.Wrap(err, "write vk.json")
	}

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

// {
//     "vk_alpha1_x": 20491192805390485299153009773594534940189261866228447918068658471970481763042,
//     "vk_alpha1_y": 9383485363053290200918347156157836566562967994039712273449902621266178545958,
//     "vk_beta2_x1": 4252822878758300859123897981450591353533073413197771768651442665752259397132,
//     "vk_beta2_x0": 6375614351688725206403948262868962793625744043794305715222011528459656738731,
//     "vk_beta2_y1": 21847035105528745403288232691147584728191162732299865338377159692350059136679,
//     "vk_beta2_y0": 10505242626370262277552901082094356697409835680220590971873171140371331206856,
//     "vk_gamma2_x1": 11559732032986387107991004021392285783925812861821192530917403151452391805634,
//     "vk_gamma2_x0": 10857046999023057135944570762232829481370756359578518086990519993285655852781,
//     "vk_gamma2_y1": 4082367875863433681332203403145435568316851327593401208105741076214120093531,
//     "vk_gamma2_y0": 8495653923123431417604973247489272438418190587263600148770280649306958101930,
//     "vk_delta2_x1": 12599857379517512478445603412764121041984228075771497593287716170335433683702,
//     "vk_delta2_x0": 7912208710313447447762395792098481825752520616755888860068004689933335666613,
//     "vk_delta2_y1": 11502426145685875357967720478366491326865907869902181704031346886834786027007,
//     "vk_delta2_y0": 21679208693936337484429571887537508926366191105267550375038502782696042114705,
//     "ax": 6424909707529041010431833767196069900905951186152453452535233785859310247091,
//     "ay": 15156427692937982705101882891732675959670219989397474519215594210140745958982,
//     "bx1": 11702600617119966915217386854353771222477427862839239072991366294351362953119,
//     "bx0": 9377668754004040279698406674069547206576290350544684455848413744271894321832,
//     "by1": 3628891305020420995628487021870577687557167953941662416598489001684202886401,
//     "by0": 2339250257289832665920974862775225721388286867501651664202401324220401621360,
//     "cx": 11664089827190113040588903049366218671264446383108882453852976389666897952784,
//     "cy": 11964654005374721149828827734828350582389212487311147825692987599394401865041,
//     "vk_ic0_x": 19918517214839406678907482305035208173510172567546071380302965459737278553528,
//     "vk_ic0_y": 7151186077716310064777520690144511885696297127165278362082219441732663131220,
//     "vk_ic1_x": 690581125971423619528508316402701520070153774868732534279095503611995849608,
//     "vk_ic1_y": 21271996888576045810415843612869789314680408477068973024786458305950370465558,
//     "vk_ic2_x": 16461282535702132833442937829027913110152135149151199860671943445720775371319,
//     "vk_ic2_y": 2814052162479976678403678512565563275428791320557060777323643795017729081887,
//     "vk_ic3_x": 4319780315499060392574138782191013129592543766464046592208884866569377437627,
//     "vk_ic3_y": 13920930439395002698339449999482247728129484070642079851312682993555105218086,
//     "vk_ic4_x": 3554830803181375418665292545416227334138838284686406179598687755626325482686,
//     "vk_ic4_y": 5951609174746846070367113593675211691311013364421437923470787371738135276998,
//     "input_0": 21705359380887563070149446940526235604214635927514568554179739874068936581326,
//     "input_1": 13630543870832548245486205251095070353732552187251924897575030362759902904563,
//     "input_2": 162293819954371680461551288230016133027961592341006905831869429197623161310,
//     "input_3": 335454208347774977187488698124679955736871218687692995704073124237365188281
// }

type VerifyingKeyJSON struct {
	VkAlpha1X  string `json:"vk_alpha1_x"`
	VkAlpha1Y  string `json:"vk_alpha1_y"`
	VkBeta2X1  string `json:"vk_beta2_x1"`
	VkBeta2X0  string `json:"vk_beta2_x0"`
	VkBeta2Y1  string `json:"vk_beta2_y1"`
	VkBeta2Y0  string `json:"vk_beta2_y0"`
	VkGamma2X1 string `json:"vk_gamma2_x1"`
	VkGamma2X0 string `json:"vk_gamma2_x0"`
	VkGamma2Y1 string `json:"vk_gamma2_y1"`
	VkGamma2Y0 string `json:"vk_gamma2_y0"`
	VkDelta2X1 string `json:"vk_delta2_x1"`
	VkDelta2X0 string `json:"vk_delta2_x0"`
	VkDelta2Y1 string `json:"vk_delta2_y1"`
	VkDelta2Y0 string `json:"vk_delta2_y0"`
	Ax         string `json:"ax"`
	Ay         string `json:"ay"`
	Bx1        string `json:"bx1"`
	Bx0        string `json:"bx0"`
	By1        string `json:"by1"`
	By0        string `json:"by0"`
	Cx         string `json:"cx"`
	Cy         string `json:"cy"`
	VkIc0X     string `json:"vk_ic0_x"`
	VkIc0Y     string `json:"vk_ic0_y"`
	VkIc1X     string `json:"vk_ic1_x"`
	VkIc1Y     string `json:"vk_ic1_y"`
	VkIc2X     string `json:"vk_ic2_x"`
	VkIc2Y     string `json:"vk_ic2_y"`
	VkIc3X     string `json:"vk_ic3_x"`
	VkIc3Y     string `json:"vk_ic3_y"`
	VkIc4X     string `json:"vk_ic4_x"`
	VkIc4Y     string `json:"vk_ic4_y"`
	Input0     string `json:"input_0"`
	Input1     string `json:"input_1"`
	Input2     string `json:"input_2"`
	Input3     string `json:"input_3"`
}

// VerifyingKeyWrapper wraps groth16.VerifyingKey to allow adding methods.
type VerifyingKeyWrapper struct {
	*groth16_bn254.VerifyingKey
}

func (vk *VerifyingKeyWrapper) WriteJSONTo(w io.Writer) error {
	vkJSON := VerifyingKeyJSON{}

	// Fill in the scalar fields
	vkJSON.VkAlpha1X = elementToStr(vk.G1.Alpha.X)
	vkJSON.VkAlpha1Y = elementToStr(vk.G1.Alpha.Y)
	vkJSON.Ax = elementToStr(vk.G1.Beta.X)

	// Fill in the complex fields like vk.G2.Beta, vk.G2.Gamma, vk.G2.Delta
	vkJSON.VkBeta2X1 = elementToStr(vk.G2.Beta.X.A1)
	vkJSON.VkBeta2X0 = elementToStr(vk.G2.Beta.X.A0)
	vkJSON.VkBeta2Y1 = elementToStr(vk.G2.Beta.Y.A1)
	vkJSON.VkBeta2Y0 = elementToStr(vk.G2.Beta.Y.A0)
	vkJSON.VkGamma2X1 = elementToStr(vk.G2.Gamma.X.A1)
	vkJSON.VkGamma2X0 = elementToStr(vk.G2.Gamma.X.A0)
	vkJSON.VkGamma2Y1 = elementToStr(vk.G2.Gamma.Y.A1)
	vkJSON.VkGamma2Y0 = elementToStr(vk.G2.Gamma.Y.A0)
	vkJSON.VkDelta2X1 = elementToStr(vk.G2.Delta.X.A1)
	vkJSON.VkDelta2X0 = elementToStr(vk.G2.Delta.X.A0)
	vkJSON.VkDelta2Y1 = elementToStr(vk.G2.Delta.Y.A1)
	vkJSON.VkDelta2Y0 = elementToStr(vk.G2.Delta.Y.A0)

	vk.NbG1()

	// Marshal the struct to JSON
	jsonData, err := json.MarshalIndent(vkJSON, "", "    ")
	if err != nil {
		return err
	}

	// Write the JSON data to the writer
	_, err = w.Write(jsonData)
	return err
}

func elementToStr(e [4]uint64) string {
	// assumes little endian, shifts each limb by 64 bits and adds to bigInt
	bigInt := new(big.Int)
	for i := len(e) - 1; i >= 0; i-- {
		bigInt.Lsh(bigInt, 64)
		bigInt.Add(bigInt, new(big.Int).SetUint64(e[i]))
	}
	return bigInt.String()
}
