export function pointToBigInt(point) {
    const [x, y] = point.toAffine();
    return [x.value, y.value];
}
export function bigIntToArray(n, k, x) {
    let mod = 1n;
    for (let idx = 0; idx < n; idx++) {
        mod = mod * 2n;
    }
    const ret = [];
    let x_temp = x;
    for (let idx = 0; idx < k; idx++) {
        ret.push((x_temp % mod).toString());
        x_temp = x_temp / mod;
    }
    return ret;
}
