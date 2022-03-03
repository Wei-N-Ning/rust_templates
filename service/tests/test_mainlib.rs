use lib_cli::mainlib;

// reference
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html

#[test]
fn test_add() {
    assert_eq!(mainlib::add(1, 2), 3);
}
