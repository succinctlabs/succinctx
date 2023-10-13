import { toHexString } from "@chainsafe/ssz";
import { ssz } from "@lodestar/types";
import { PointG1, PointG2 } from "@noble/bls12-381";
import { toLittleEndian, hexToBigIntArray, truncateHexPrefix } from "./ssz";
import { bigIntToArray, pointToBigInt } from "./bigint";
export function stringifyCircomInput(input) {
    /* eslint-disable */
    // @ts-ignore
    BigInt.prototype.toJSON = function () {
        return this.toString();
    };
    /* eslint-enable */
    return JSON.stringify(input);
}
export class CircomSerializer {
    buffer;
    constructor() {
        this.buffer = {};
    }
    writeBeaconBlockHeader(prefix, header) {
        this.writeNumberAsBytes32(`${prefix}Slot`, header.slot);
        this.writeNumberAsBytes32(`${prefix}ProposerIndex`, header.proposerIndex);
        this.writeBytes32(`${prefix}ParentRoot`, header.parentRoot);
        this.writeBytes32(`${prefix}StateRoot`, header.stateRoot);
        this.writeBytes32(`${prefix}BodyRoot`, header.bodyRoot);
    }
    writeBeaconBlock(prefix, header) {
        this.writeNumberAsBytes32(`${prefix}Slot`, header.slot);
        this.writeNumberAsBytes32(`${prefix}ProposerIndex`, header.proposerIndex);
        this.writeBytes32(`${prefix}ParentRoot`, header.parentRoot);
        this.writeBytes32(`${prefix}StateRoot`, header.stateRoot);
        const bodyRoot = ssz.capella.BeaconBlockBody.hashTreeRoot(header.body);
        this.writeBytes32(`${prefix}BodyRoot`, bodyRoot);
    }
    writeG1PointAsBytes(name, point) {
        this.buffer[name] = hexToBigIntArray(toHexString(point));
    }
    writeG1PointsAsBytes(name, points) {
        this.buffer[name] = points.map((p) => hexToBigIntArray(toHexString(p)));
    }
    writeG1PointAsBigInt(name, point, n = 55, k = 7) {
        const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
        const bigints = pointToBigInt(p);
        this.buffer[name] = [
            bigIntToArray(n, k, bigints[0]),
            bigIntToArray(n, k, bigints[1]),
        ];
    }
    writeG1PointXAsBigInt(name, point, n = 55, k = 7) {
        const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
        const bigints = pointToBigInt(p);
        this.buffer[name] = bigIntToArray(n, k, bigints[0]);
    }
    writeG1PointYAsBigInt(name, point, n = 55, k = 7) {
        const p = PointG1.fromHex(truncateHexPrefix(toHexString(point)));
        const bigints = pointToBigInt(p);
        this.buffer[name] = bigIntToArray(n, k, bigints[1]);
    }
    writeG1PointsXAsBigInt(name, points, n = 55, k = 7) {
        this.buffer[name] = points.map((p) => {
            const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
            const bigints = pointToBigInt(point);
            return bigIntToArray(n, k, bigints[0]);
        });
    }
    writeG1PointsYAsBigInt(name, points, n = 55, k = 7) {
        this.buffer[name] = points.map((p) => {
            const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
            const bigints = pointToBigInt(point);
            return bigIntToArray(n, k, bigints[1]);
        });
    }
    writeG1PointsAsBigInt(name, points, n = 55, k = 7) {
        this.buffer[name] = points.map((p) => {
            const point = PointG1.fromHex(truncateHexPrefix(toHexString(p)));
            const bigints = pointToBigInt(point);
            return [bigIntToArray(n, k, bigints[0]), bigIntToArray(n, k, bigints[1])];
        });
    }
    writeG2Point(name, point) {
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
    writeMerkleBranch(name, data) {
        const branch = [];
        for (let i = 0; i < data.length; i++) {
            branch.push(hexToBigIntArray(toHexString(data[i])));
        }
        this.buffer[name] = branch;
    }
    writeBitArray(name, data) {
        const bits = data.toBoolArray().map((x) => (x ? 1n : 0n));
        this.writeBigIntArray(name, bits);
    }
    writeBytes32(name, data) {
        this.buffer[name] = hexToBigIntArray(toHexString(data));
    }
    writeNumberAsBytes32(name, data) {
        this.buffer[name] = hexToBigIntArray(toHexString(toLittleEndian(data)));
    }
    writeBigInt(name, data) {
        this.buffer[name] = data;
    }
    writeBigIntArray(name, data) {
        this.buffer[name] = data;
    }
    flush() {
        const buffer = this.buffer;
        this.buffer = {};
        return buffer;
    }
}
