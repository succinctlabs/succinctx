/// <reference types="node" />
import { CircomInput } from "./serializer";
export type ProofData = {
    witness: CircomInput;
    outputBytes: Uint8Array;
};
export declare abstract class Circuit {
    constructor();
    abstract generateProofData(inputBytes: Buffer): Promise<ProofData>;
    abstract circuitName(): string;
    build(snarkjsPath: string, circomPath: string, ptauPath: string): void;
    prove(rapidsnarkPath: string, inputJsonPath: string): Promise<void>;
    entrypoint(): void;
}
//# sourceMappingURL=circuit.d.ts.map