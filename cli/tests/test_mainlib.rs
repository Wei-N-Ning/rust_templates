// TODO: replace this file with the real library integration tests

use lib_cli::prelude::*;

// reference
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
