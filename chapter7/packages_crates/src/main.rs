fn main() {
    println!("main");

    // binary crate:
    // main defined at src/main.rs
    // can have multiple binaries at src/bin/xyz.rs

    // ensure cargo knows what binary to run when doing `cargo run`.
    // After specifying default-run = "b1" in Cargo.toml this error
    // went away and b1 was run.
    /*
    error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
    available binaries: b1, b2, packages_crates

    https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field
    */

    // library crate:
    // various non-'main' functions in src/lib/xyz.rs

    // both:
    // binary & library
}
