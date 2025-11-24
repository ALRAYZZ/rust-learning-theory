// Each file in the `tests` directory is compiled as a separate crate.
// Therefore, to use the library code, we need to import it explicitly.

// Cargo will only compile and run the files in the `tests` directory when we run `cargo test`.
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}