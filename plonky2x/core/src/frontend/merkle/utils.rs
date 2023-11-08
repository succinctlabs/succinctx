/// Computes the ceiling of the base 2 log of a `usize`.
pub fn log2_ceil_usize(x: usize) -> usize {
    if x <= 1 {
        // log2(0) and log2(1) are both 0.
        return 0;
    }

    let mut result = 0;
    // Subtract 1 to ensure rounding up for powers of 2.
    let mut value = x - 1;

    while value > 0 {
        value >>= 1;
        result += 1;
    }

    result as usize
}
