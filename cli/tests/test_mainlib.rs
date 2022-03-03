use lib_cli::mainlib;

#[test]
fn test_add() {
    assert_eq!(mainlib::add(1, 2), 3);
}
