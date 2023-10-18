import { toHexString, BitArray } from "@chainsafe/ssz";
import { capella, phase0, ssz } from "@lodestar/types";
import { PointG1, PointG2 } from "@noble/bls12-381";

import { toLittleEndian, hexToBigIntArray, truncateHexPrefix } from "./ssz.js";
import { bigIntToArray, pointToBigInt } from "./bigint.js";

export type CircomElement = string | bigint | number | CircomElement[];

export type CircomInput = {
  [index: string]: CircomElement;
};

export function stringifyCircomInput(input: CircomInput): string {
  /* eslint-disable */
  // @ts-ignore
  BigInt.prototype.toJSON = function () {
    return this.toString();
  };
  /* eslint-enable */
  return JSON.stringify(input);
}

export class CircomSerializer {
  buffer: CircomInput;

  constructor() {
    this.buffer = {};
  }

  writeBeaconBlockHeader(prefix: string, header: phase0.BeaconBlockHeader) {
    this.writeNumberAsBytes32(`${prefix}Slot`, header.slot);
    this.writeNumberAsBytes32(`${prefix}ProposerIndex`, header.proposerIndex);
    this.writeBytes32(`${prefix}ParentRoot`, header.parentRoot);
    this.writeBytes32(`${prefix}StateRoot`, header.stateRoot);
    this.writeBytes32(`${prefix}BodyRoot`, header.bodyRoot);
  }

  writeBeaconBlock(prefix: string, header: capella.BeaconBlock) {
    this.writeNumberAsBytes32(`${prefix}Slot`, header.slot);
    this.writeNumberAsBytes32(`${prefix}ProposerIndex`, header.proposerIndex);
    this.writeBytes32(`${prefix}ParentRoot`, header.parentRoot);
    this.writeBytes32(`${prefix}StateRoot`, header.stateRoot);
    const bodyRoot = ssz.capella.BeaconBlockBody.hashTreeRoot(header.body);
    this.writeBytes32(`${prefix}BodyRoot`, bodyRoot);
  }

  writeG1PointAsBytes(name: string, point: Uint8Array) {
    this.buffer[name] = hexToBigIntArray(toHexString(point));
  }

  writeG1PointsAsBytes(name: string, points: Uint8Array[]) {
    this.buffer[name] = points.map((p) => hexToBigIntArray(toHexString(p)));
  }

  writeG1PointAsBigInt(name: string, point: Uint8Array, n = 55, k = 7) {
    const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
    const bigints = pointToBigInt(p);
    this.buffer[name] = [
      bigIntToArray(n, k, bigints[0]),
      bigIntToArray(n, k, bigints[1]),
    ];
  }

  writeG1PointXAsBigInt(name: string, point: Uint8Array, n = 55, k = 7) {
    const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
    const bigints = pointToBigInt(p);
    this.buffer[name] = bigIntToArray(n, k, bigints[0]);
  }

  writeG1PointYAsBigInt(name: string, point: Uint8Array, n = 55, k = 7) {
    const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
    const bigints = pointToBigInt(p);
    this.buffer[name] = bigIntToArray(n, k, bigints[1]);
  }

  writeG1PointsXAsBigInt(name: string, points: Uint8Array[], n = 55, k = 7) {
    this.buffer[name] = points.map((p) => {
      const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
      const bigints = pointToBigInt(point);
      return bigIntToArray(n, k, bigints[0]);
    });
  }

  writeG1PointsYAsBigInt(name: string, points: Uint8Array[], n = 55, k = 7) {
    this.buffer[name] = points.map((p) => {
      const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
      const bigints = pointToBigInt(point);
      return bigIntToArray(n, k, bigints[1]);
    });
  }

  writeG1PointsAsBigInt(name: string, points: Uint8Array[], n = 55, k = 7) {
    this.buffer[name] = points.map((p) => {
      const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
      const bigints = pointToBigInt(point);
      return [bigIntToArray(n, k, bigints[0]), bigIntToArray(n, k, bigints[1])];
    });
  }

  writeG2Point(name: string, point: Uint8Array) {
    const p = PointG2.fromSignature(truncateHexPrefix(toHexString(point)));
    p.assertValidity();
    this.buffer[name] = [
      [
        bigIntToArray(55, 7, p.toAffine()[0].c0.value),
        bigIntToArray(55, 7, p.toAffine()[0].c1.value),
      ],
      [
        bigIntToArray(55, 7, p.toAffine()[1].c0.value),
        bigIntToArray(55, 7, p.toAffine()[1].c1.value),
      ],
    ];
  }

  writeMerkleBranch(name: string, data: Uint8Array[]) {
    const branch = [];
    for (let i = 0; i < data.length; i++) {
      branch.push(hexToBigIntArray(toHexString(data[i])));
    }
    this.buffer[name] = branch;
  }

  writeBitArray(name: string, data: BitArray) {
    const bits = data.toBoolArray().map((x) => (x ? 1n : 0n));
    this.writeBigIntArray(name, bits);
  }

  writeBytes32(name: string, data: Uint8Array) {
    this.buffer[name] = hexToBigIntArray(toHexString(data));
  }

  writeNumberAsBytes32(name: string, data: number) {
    this.buffer[name] = hexToBigIntArray(toHexString(toLittleEndian(data)));
  }

  writeBigInt(name: string, data: bigint) {
    this.buffer[name] = data;
  }

  writeBigIntArray(name: string, data: bigint[]) {
    this.buffer[name] = data;
  }

  flush(): CircomInput {
    const buffer = this.buffer;
    this.buffer = {};
    return buffer;
  }
}
