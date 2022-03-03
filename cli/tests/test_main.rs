use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
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
