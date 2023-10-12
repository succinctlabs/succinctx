/// <reference types="node" />
/// <reference types="node" />
import crypto from "crypto";
export declare function encodeGroth16Proof(proof: any): string;
export declare function executeCommand(command: string): void;
export declare function readableTime(ms: number): string;
export declare function sha256(input: crypto.BinaryLike): Buffer;
/**
 * Computes the input or output hash given a variable length byte array
 * @param input variable length input or output bytes
 * @returns the sha256 hash of the input as a bigint truncated to lower 253 bits
 */
export declare function ioHash(input: crypto.BinaryLike): bigint;
//# sourceMappingURL=util.d.ts.map