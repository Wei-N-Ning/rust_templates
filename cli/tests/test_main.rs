// TODO: replace this file with the real CLI integration tests

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::path::PathBuf;
use std::process::Command; // Run programs

// reference
// https://rust-cli.github.io/book/tutorial/testing.html

#[test]
fn insufficient_args_expect_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli")?;
    cmd.assert().failure();
    Ok(())
}

#[test]
fn expect_success_and_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli")?;
    cmd.arg("--name").arg("asd");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
    Ok(())
}

#[test]
fn read_testdata_file() -> Result<(), Box<dyn std::error::Error>> {
    let filename: PathBuf = [env!("CARGO_MANIFEST_DIR"), "testdata", "foo.txt"]
        .iter()
        .collect();
    let s = std::fs::read_to_string(&filename)?;
    let mut cmd = Command::cargo_bin("cli")?;
    cmd.arg("--name").arg(s);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello foo"));
    Ok(())
}
