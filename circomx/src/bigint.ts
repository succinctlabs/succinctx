import { PointG1 } from "@noble/bls12-381";

export function pointToBigInt(point: PointG1): [bigint, bigint] {
  const [x, y] = point.toAffine();
  return [x.value, y.value];
}

export function bigIntToArray(n: number, k: number, x: bigint) {
  let mod = 1n;
  for (let idx = 0; idx < n; idx++) {
    mod = mod * 2n;
  }

  const ret: string[] = [];
  let x_temp: bigint = x;
  for (let idx = 0; idx < k; idx++) {
    ret.push((x_temp % mod).toString());
    x_temp = x_temp / mod;
  }
  return ret;
}
