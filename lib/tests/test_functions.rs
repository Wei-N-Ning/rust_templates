use lib::functions;

// reference
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html

#[test]
fn test_add() {
    assert_eq!(functions::add(1, 2), 3);
}
