pub fn pos_exponent(base: usize, power: usize) -> usize {
    if power == 0 {
        return 1
    }

    let mut result: usize = base;
    for _i in 0..power {
        result = result * base;
    }

    return result
}