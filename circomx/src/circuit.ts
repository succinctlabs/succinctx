import { toHexString } from "@chainsafe/ssz";
import dotenv from "dotenv";
import fs from "fs";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { encodeGroth16Proof, executeCommand } from "./util.js";
import { CircomInput } from "./serializer.js";

const NODE_OPTIONS =
  "--huge-max-old-generation-size --max-old-space-size=2048000 --initial-old-space-size=2048000 --no-global-gc-scheduling --no-incremental-marking --max-semi-space-size=2048000 --initial-heap-size=2048000 --expose-gc";

// @ts-ignore
BigInt.prototype.toJSON = function () {
  return this.toString();
};

export type ProofData = {
  witness: CircomInput;
  outputBytes: Uint8Array;
};

export abstract class Circuit {
  constructor() {}

  abstract generateProofData(inputBytes: Buffer): Promise<ProofData>;

  abstract circuitName(): string;

  build(
    snarkjsPath: string,
    circomPath: string,
    ptauPath: string,
    noZkey: boolean
  ) {
    const circuit = this.circuitName();
    console.log(
      `Building ${circuit} with args (${snarkjsPath} ${circomPath} ${ptauPath} ${noZkey})`
    );

    // Create build dir if not exists
    if (!fs.existsSync("build")) {
      fs.mkdirSync("build");
    }

    executeCommand(
      `${circomPath} circuits/${circuit}.circom --O1 --r1cs --sym --c --output build`
    );
    const circuitName = circuit === "main" ? "main_c" : circuit;
    const buildDirName = `${circuitName}_cpp`;
    executeCommand(`make -C build/${buildDirName}/`);
    executeCommand(
      `cp build/${buildDirName}/${circuitName} build/${circuitName}`
    );
    executeCommand(
      `cp build/${buildDirName}/${circuitName}.dat build/${circuitName}.dat`
    );
    // Tar build dir
    executeCommand(
      `tar -czf build/${buildDirName}.tar.gz -C build ${buildDirName}`
    );
    // Remove build dir
    executeCommand(`rm -rf build/${buildDirName}`);
    if (!noZkey) {
      executeCommand(
        `node ${NODE_OPTIONS} ${snarkjsPath} zkey new build/${circuitName}.r1cs ${ptauPath} build/p1.zkey`
      );
      executeCommand(
        `node ${NODE_OPTIONS} ${snarkjsPath} zkey export verificationkey build/p1.zkey build/vkey.json`
      );
      executeCommand(
        `node ${NODE_OPTIONS} ${snarkjsPath} zkey export solidityverifier build/p1.zkey build/FunctionVerifier.sol`
      );

      // Replace first line of FunctionVerifier.sol with "pragma solidity ^0.8.0;"
      let solidityVerifier = fs.readFileSync(
        "build/FunctionVerifier.sol",
        "utf8"
      );
      solidityVerifier = solidityVerifier.replaceAll("calldataload", "mload");
      solidityVerifier = solidityVerifier.replaceAll("calldata", "memory");
      solidityVerifier = solidityVerifier.replaceAll(
        "_pB, _pC",
        // for some reason, uint256[2][2] memory _pB has two words (two lengths?) prepended to it
        // and calldata doesn't have it
        "add(_pB, 64), _pC"
      );
      solidityVerifier = solidityVerifier.replaceAll(
        "pragma solidity >=0.7.0 <0.9.0;",
        "pragma solidity ^0.8.16;"
      );
      solidityVerifier += `

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, Groth16Verifier {

    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) =
            abi.decode(_proof, (uint256[2], uint256[2][2], uint256[2]));

        uint256[2] memory input = [uint256(_outputHash), uint256(_inputHash)];
        input[0] = input[0] & ((1 << 253) - 1);
        input[1] = input[1] & ((1 << 253) - 1);

        return verifyProof(a, b, c, input);
    }

    function verificationKeyHash() external pure returns (bytes32) {
        bytes memory left;
        bytes memory right;
        {
            left = abi.encode(alphax, alphay, betax1, betax2, betay1, betay2);
        }
        {
            right = abi.encode(gammax1, gammax2, gammay1, gammay2, deltax1, deltax2, deltay1, deltay2);
        }
        return keccak256(abi.encode(left, right));
    }
}
`;
      fs.writeFileSync("build/FunctionVerifier.sol", solidityVerifier);
    }
  }

  async prove(rapidsnarkPath: string, inputJsonPath: string, zkeyName: string) {
    const circuit = this.circuitName();
    const circuitName = circuit === "main" ? "main_c" : circuit;

    const data = fs.readFileSync(inputJsonPath, "utf8");
    const jsonData = JSON.parse(data);
    console.log(jsonData);

    let hexString = jsonData.data.input;

    // Remove '0x' prefix if it exists
    if (hexString.startsWith("0x")) {
      hexString = hexString.slice(2);
    }

    const buffer = Buffer.from(hexString, "hex");

    const proofData = await this.generateProofData(buffer);

    fs.writeFileSync("witness.json", JSON.stringify(proofData.witness));

    executeCommand(`./build/${circuitName} witness.json witness.wtns`);
    executeCommand(
      `${rapidsnarkPath} build/${zkeyName} witness.wtns proof.json public.json`
    );

    const publicData = fs.readFileSync("public.json", "utf8");
    const publicJsonData = JSON.parse(publicData);
    console.log(publicJsonData);

    const proofDataFile = fs.readFileSync("proof.json", "utf8");
    const proofJsonData = JSON.parse(proofDataFile);

    // // TODO: sanity check circuit inputs
    // const circuitGeneratedInputs = publicJsonData.map((v: string) => {
    //   const hex = BigInt(v).toString(16);
    //   const paddedLen = Math.ceil(hex.length / 2) * 2;
    //   return hex.padStart(paddedLen, "0");
    // });

    const outputBytes = toHexString(proofData.outputBytes);

    const proofBytes = encodeGroth16Proof(proofJsonData);
    const outputJson = {
      type: "res_bytes",
      data: {
        proof: proofBytes,
        output: outputBytes,
      },
    };

    fs.writeFileSync("output.json", JSON.stringify(outputJson));
    console.log("Done");
  }

  entrypoint() {
    dotenv.config();
    yargs(hideBin(process.argv))
      .command(
        "build",
        "Run build commands",
        (yargs) => {
          yargs
            .option("snarkjs", {
              describe: "Path to snarkjs cli.js",
              type: "string",
              default: "/root/snarkjs/cli.js",
            })
            .option("circom", {
              describe: "circom executable",
              type: "string",
              default: "circom",
            })
            .option("ptau", {
              describe: "Path to powersOfTau.ptau",
              type: "string",
              default: "/root/powersOfTau.ptau",
            })
            .option("skip-zkey", {
              describe: "Don't generate zkey",
              type: "boolean",
              default: false,
            });
        },
        (args) => {
          const snarkjsPath = args.snarkjs as string;
          const circomPath = args.circom as string;
          const ptauPath = args.ptau as string;
          const noZkey = args["skip-zkey"] as boolean;
          this.build(snarkjsPath, circomPath, ptauPath, noZkey);
        }
      )
      .command(
        "prove",
        "Run prove commands",
        (yargs) => {
          yargs
            .option("input-json", {
              describe: "Path to the input JSON file",
              type: "string",
              default: "input.json",
            })
            .option("rapidsnark", {
              describe: "Rapidsnark command",
              type: "string",
              default: "rapidsnark",
            })
            .option("zkey", {
              describe: "Name of the zkey to use",
              type: "string",
              default: "p1.zkey",
            });
        },
        async (args) => {
          const rapidsnarkPath = args.rapidsnark as string;
          const inputJsonPath = args["input-json"] as string;
          const zkeyName = args["zkey"] as string;

          await this.prove(rapidsnarkPath, inputJsonPath, zkeyName);
        }
      )
      .demandCommand(1, "You need to provide a command (either build or prove)")
      .parse();
  }
}
