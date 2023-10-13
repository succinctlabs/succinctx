import { toHexString } from "@chainsafe/ssz";
import { ssz } from "@lodestar/types";
export function toLittleEndian(x) {
    const y = new Uint8Array(32);
    for (let i = 0; i < 32; i++) {
        y[i] = x & 0xff;
        x = x >> 8;
    }
    return y;
}
export function toLittleEndianFromBigInt(x) {
    const mask = BigInt(0xff);
    const y = new Uint8Array(32);
    for (let i = 0; i < 32; i++) {
        y[i] = Number(x & mask);
        x = x >> 8n;
    }
    return y;
}
export function toBigIntBEFromBytes32(x) {
    let sum = BigInt(0);
    for (let i = 0; i < 32; i++) {
        sum += BigInt(x[i]);
        if (i < 31) {
            sum *= BigInt(2 ** 8);
        }
    }
    return sum;
}
export function toBigIntFromBytes32(x) {
    let sum = BigInt(0);
    for (let i = 0; i < 32; i++) {
        sum += BigInt(x[32 - i - 1]);
        if (i < 31) {
            sum *= BigInt(2 ** 8);
        }
    }
    return sum;
}
export function get253BitMask() {
    let x = BigInt(1);
    x = x << 253n;
    x = x - 1n;
    return x;
}
export function computeBitSum(bits) {
    return BigInt(bits
        .toBoolArray()
        .map((x) => (x ? Number(1) : Number(0)))
        .reduce((x, y) => x + y));
}
export function truncateHexPrefix(str) {
    if (str.startsWith("0x")) {
        str = str.slice(2);
    }
    return str;
}
export function hexToBytes(hex) {
    hex = truncateHexPrefix(hex);
    const array = new Uint8Array(hex.length / 2);
    for (let i = 0; i < array.length; i++) {
        const j = i * 2;
        const hexByte = hex.slice(j, j + 2);
        if (hexByte.length !== 2) {
            throw Error("Invalid byte sequence");
        }
        const byte = Number.parseInt(hexByte, 16);
        if (Number.isNaN(byte) || byte < 0) {
            throw Error("Invalid byte sequence");
        }
        array[i] = byte;
    }
    return array;
}
export function hexToBigIntArray(hex) {
    hex = truncateHexPrefix(hex);
    const array = [];
    for (let i = 0; i < hex.length / 2; i++) {
        const j = i * 2;
        const hexByte = hex.slice(j, j + 2);
        if (hexByte.length !== 2) {
            throw Error("Invalid byte sequence");
        }
        const byte = Number.parseInt(hexByte, 16);
        if (Number.isNaN(byte) || byte < 0) {
            throw Error("Invalid byte sequence");
        }
        array.push(BigInt(byte));
    }
    return array;
}
export function hexToBits(hex) {
    const bytes = hexToBytes(hex);
    const array = new Array(bytes.length * 8);
    for (let i = 0; i < bytes.length; i++) {
        let value = bytes[i];
        for (let j = 0; j < 8; j++) {
            array[i * 8 + j] = value & 0x1;
            value = value >> 1;
        }
    }
    return array;
}
export function restoreMerkleRoot(leaf, index, branch) {
    let value = leaf;
    for (let i = 0; i < branch.length; i++) {
        if ((index / 2n ** BigInt(i)) % 2n === 1n) {
            value = hashPair(branch[i], value);
        }
        else {
            value = hashPair(value, branch[i]);
        }
    }
    return value;
}
export function isValidMerkleBranch(leaf, index, branch, root) {
    const restored = restoreMerkleRoot(leaf, index, branch);
    return toHexString(restored) === toHexString(root);
}
export function hashBeaconBlockHeader(header) {
    return ssz.phase0.BeaconBlockHeader.hashTreeRoot(header);
}
export function hashSyncCommittee(syncCommittee) {
    return ssz.altair.SyncCommittee.hashTreeRoot(syncCommittee);
}
export function hashPair(x, y) {
    if (x.length !== 32) {
        throw Error("Expected first input to be of length 32");
    }
    if (y.length !== 32) {
        throw Error("Expected second input to be of length 32");
    }
    return ssz.phase0.SigningData.hashTreeRoot({ objectRoot: x, domain: y });
}
export function computeDomain(forkVersion, genesisValidatorsRoot) {
    const paddedForkVersion = new Uint8Array(32);
    for (let i = 0; i < 4; i++) {
        paddedForkVersion[i] = forkVersion[i];
    }
    const syncDomainRoot = hashPair(paddedForkVersion, genesisValidatorsRoot);
    return hexToBytes("0x07000000" + toHexString(syncDomainRoot).slice(2, -8));
}
