pub fn log2_ceil_usize(x: usize) -> usize {
    if x <= 1 {
        return 0; // log2(0) and log2(1) are both 0
    }

    let mut result = 0;
    let mut value = x - 1; // Subtract 1 to ensure rounding up for powers of 2

    while value > 0 {
        value >>= 1; // Right shift by 1, equivalent to dividing by 2
        result += 1;
    }

    result as usize
}
