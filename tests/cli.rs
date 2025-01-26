use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("snipcomp")?;

    cmd.arg("-s").arg("examples/testspec.md");
    cmd.arg("-e").arg("toscaexamples");
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
        .stderr(predicate::str::contains("Usage: snipcomp [OPTIONS] --spec-path <SPEC_PATH> --example-path <EXAMPLE_PATH>"));

    Ok(())
}

#[test]
fn file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("snipcomp")?;

    cmd.arg("-s").arg("examples/not_there.md");
    cmd.arg("-e").arg("toscaexamples");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("examples/not_there"));
    Ok(())
}

#[test]
fn file_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("snipcomp")?;

    cmd.arg("-s").arg("examples/testspec.md");
    cmd.arg("-e").arg("toscaexamples");
    cmd.arg("-o").arg("report");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unable to match block:"));
    Ok(())
}