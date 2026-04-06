// if don't add to libraries ../Cargo.toml then get error: no external crate `rand`
use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Run only these tests from workspace root with `cargo test -p add_one`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
