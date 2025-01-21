use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("snipcomp")?;

    cmd.arg("-s").arg("testspec.md");
    cmd.arg("-e").arg("examples");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tosca_definitions_version: tosca_2_0"));

    Ok(())
}
#[test]
fn wrong_args () -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("snipcomp")?;

    cmd.arg("-x").arg("notthere");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: snipcomp --spec-path <SPEC_PATH> --example-path <EXAMPLE_PATH>"));

    Ok(())
}