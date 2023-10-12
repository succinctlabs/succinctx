import { execSync } from "child_process";
import { ethers } from "ethers";
import crypto from "crypto";
import * as ssz from "./ssz";

export function encodeGroth16Proof(proof: any): string {
  // Flatten the proof values into a single array
  // rapidsnark outputs the contents of b0 and b1 in reverse order from what Solidity verifier expects
  // so we swap them here
  const b0 = proof.pi_b[0].slice(0, 2);
  b0.reverse();
  const b1 = proof.pi_b[1].slice(0, 2);
  b1.reverse();
  const values = [
    ...proof.pi_a.slice(0, 2), // We only need the first two elements, ignoring the third "1"
    ...b0,
    ...b1,
    ...proof.pi_c.slice(0, 2),
  ].map((v) => ethers.BigNumber.from(v));

  // Encode using ethers' ABI coder
  const encoded = ethers.utils.defaultAbiCoder.encode(
    ["uint256[2]", "uint256[2][2]", "uint256[2]"],
    [
      values.slice(0, 2),
      [values.slice(2, 4), values.slice(4, 6)],
      values.slice(6, 8),
    ]
  );

  return encoded;
}

export function executeCommand(command: string) {
  console.log(`Executing: ${command}`);
  const startTime = Date.now();
  try {
    execSync(command, { stdio: "inherit" });
  } catch (e) {
    console.log("Failed to execute command");
    console.log(e);
    process.exit(1);
  } finally {
    const endTime = Date.now();
    console.log(`Command took ${readableTime(endTime - startTime)}`);
  }
}

export function readableTime(ms: number): string {
  if (ms < 1000) {
    return `${ms} ms`;
  }
  const sec = ms / 1000;
  if (sec < 60) {
    return `${sec} sec`;
  }
  const min = sec / 60;
  if (min < 60) {
    return `${min} min`;
  }
  const hr = min / 60;
  return `${hr} hr`;
}

export function sha256(input: crypto.BinaryLike): Buffer {
  const hasher = crypto.createHash("sha256");
  hasher.update(input);
  return hasher.digest();
}

/**
 * Computes the input or output hash given a variable length byte array
 * @param input variable length input or output bytes
 * @returns the sha256 hash of the input as a bigint truncated to lower 253 bits
 */
export function ioHash(input: crypto.BinaryLike): bigint {
  const sha = sha256(input);
  return ssz.toBigIntBEFromBytes32(sha) & ssz.get253BitMask();
}
