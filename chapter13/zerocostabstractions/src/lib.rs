// zero_cost_abstraction uses Iterators and the resulting compiled code is just as fast
// as any for loop.
// In fact the for loop gets "unrolled" as compiler knows there are 12 iterations so writes out 12
// iterations in similar assembly code.
// No need to fear iterators.
// Confusing to start with but cleanliness makes it easier to reason about when familiar.
// Closures and Iterators are Rust features inspired by functional programming language ideas, all
// without runtime performance being affected.
pub fn zero_cost_abstarction(buffer: &mut [i32], coefficients: [i64; 12], qlp_shift: i16) {
    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
