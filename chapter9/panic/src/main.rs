fn main() {
    // panic!("crash and burn");
    /*
    thread 'main' panicked at src/main.rs:2:5:
    crash and burn
    */

    let v = vec![1, 2, 3];

    v[99];
    /*
    thread 'main' panicked at src/main.rs:4:6:
    index out of bounds: the len is 3 but the index is 99
    */

    /* RUST_BACKTRACE=1 cargo run
    thread 'main' panicked at src/main.rs:10:6:
    index out of bounds: the len is 3 but the index is 99
    stack backtrace:
       0: rust_begin_unwind
       1: core::panicking::panic_fmt
       2: core::panicking::panic_bounds_check
       3: <usize as core::slice::index::SliceIndex<[T]>>::index
                 at /build/rustc-1.82.0-src/library/core/src/slice/index.rs:302:10
       4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
                 at /build/rustc-1.82.0-src/library/core/src/slice/index.rs:16:9
       5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
                 at /build/rustc-1.82.0-src/library/alloc/src/vec/mod.rs:2920:9
       6: panic::main
                 at ./src/main.rs:10:6
       7: core::ops::function::FnOnce::call_once
                 at /build/rustc-1.82.0-src/library/core/src/ops/function.rs:250:5
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    */
}
