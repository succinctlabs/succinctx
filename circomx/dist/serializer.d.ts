import { BitArray } from "@chainsafe/ssz";
import { capella, phase0 } from "@lodestar/types";
export type CircomElement = string | bigint | number | CircomElement[];
export type CircomInput = {
    [index: string]: CircomElement;
};
export declare function stringifyCircomInput(input: CircomInput): string;
export declare class CircomSerializer {
    buffer: CircomInput;
    constructor();
    writeBeaconBlockHeader(prefix: string, header: phase0.BeaconBlockHeader): void;
    writeBeaconBlock(prefix: string, header: capella.BeaconBlock): void;
    writeG1PointAsBytes(name: string, point: Uint8Array): void;
    writeG1PointsAsBytes(name: string, points: Uint8Array[]): void;
    writeG1PointAsBigInt(name: string, point: Uint8Array, n?: number, k?: number): void;
    writeG1PointXAsBigInt(name: string, point: Uint8Array, n?: number, k?: number): void;
    writeG1PointYAsBigInt(name: string, point: Uint8Array, n?: number, k?: number): void;
    writeG1PointsXAsBigInt(name: string, points: Uint8Array[], n?: number, k?: number): void;
    writeG1PointsYAsBigInt(name: string, points: Uint8Array[], n?: number, k?: number): void;
    writeG1PointsAsBigInt(name: string, points: Uint8Array[], n?: number, k?: number): void;
    writeG2Point(name: string, point: Uint8Array): void;
    writeMerkleBranch(name: string, data: Uint8Array[]): void;
    writeBitArray(name: string, data: BitArray): void;
    writeBytes32(name: string, data: Uint8Array): void;
    writeNumberAsBytes32(name: string, data: number): void;
    writeBigInt(name: string, data: bigint): void;
    writeBigIntArray(name: string, data: bigint[]): void;
    flush(): CircomInput;
}
//# sourceMappingURL=serializer.d.ts.map