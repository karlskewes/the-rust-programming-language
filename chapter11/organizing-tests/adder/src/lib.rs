pub fn add_two(left: u64) -> u64 {
    left + 2
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

// Unit tests are in same file as library code so require annotation to avoid
// being included in compiled binary, only compile and run when `test`
// configuration option (`#[cfg(<opt>)]`) is run, i.o.w: `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        // can test private internal functions if want.
        // `use super::*` brings everything into scope.
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
